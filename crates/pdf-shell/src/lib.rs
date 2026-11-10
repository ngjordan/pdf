//! The PDF app's shell: the window-independent half of the desktop app (`apps/iridium-pdf`), as a
//! library, so one host can embed it. Process-wide things (logging, GPU choice, single-instance
//! election, the updater, `run_native`) stay in the app.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::cell::RefCell;
use std::rc::Rc;

use iridium_pdf_ui_egui::{OsEventsFn, PdfApp};

mod control;
#[cfg(target_os = "windows")]
mod motion;
mod reopen;
mod startup;
mod text_scale;

use control::open_control_channel;

/// The app had a former upstream name; settings saved then are under this key.
const LEGACY_STORAGE_KEY: &str = "printcraft";

/// The app icon the Windows taskbar and Alt+Tab show (`ICON_BIG`).
#[cfg(target_os = "windows")]
const APP_ICON_PNG: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ph.pdf.png");

/// Connects the queue of files other launches or the OS hand over (`PdfApp::os_events`) once the
/// window exists; the flag is whether the window is a native Wayland one.
pub type OsEventsConnect = Rc<dyn Fn(&egui::Context, bool) -> OsEventsFn>;

/// What the app was started with. `Default` is a bare launch: no files, no options.
#[derive(Clone, Default)]
pub struct Launch {
    pub files: Vec<String>,
    /// View options (`--page 3`, `--zoom 150`, …), applied after the files open.
    pub options: Vec<(String, String)>,
    /// `--control <file>`: where to publish the UI control channel.
    pub control_file: Option<String>,
    pub create_images: bool,
    /// `--combine`: the files go to the Combine files tab.
    pub combine: bool,
    pub integrated: bool,
    /// The window buttons the app draws, when the desktop leaves the frame to it.
    pub window_controls: Option<iridium_pdf_ui_egui::window_controls::Layout>,
    pub two_paged: bool,
    /// Files handed over by other launches or the OS (Apple events); `None` takes none.
    pub os_events: Option<OsEventsConnect>,
    /// Set by [`Shell::new`] when the launch can't go on (`--control` couldn't be published): the
    /// host must stop the app with this message.
    pub startup_error: Rc<RefCell<Option<String>>>,
}

/// The PDF app, ready to be drawn into any region.
pub struct Shell {
    app: PdfApp,
}

impl Shell {
    /// Build the shell. `cc` is eframe's creation context; `launch` carries the files/flags the
    /// app was started with.
    pub fn new(cc: &eframe::CreationContext<'_>, launch: Launch) -> Self {
        let Launch { files, options, control_file, create_images, combine, integrated, window_controls, two_paged, os_events, startup_error } =
            launch;
        let mut app = PdfApp::new();
        // First start (or settings from before Units existed): the region's unit.
        app.units = iridium_pdf_ui_egui::units::Unit::system();
        #[cfg(target_os = "linux")]
        app.enable_desktop_theme();
        // The system's reopen-windows choice (macOS) starts the preference; a saved choice replaces it.
        app.reopen_last_session = reopen::system_default();
        if let Some(json) = cc.storage.and_then(|s| s.get_string("iridium_pdf").or_else(|| s.get_string(LEGACY_STORAGE_KEY))) {
            app.restore(&json);
        }
        app.window_state = iridium_pdf_ui_egui::window_state::WindowState::started_with(cc.storage);
        app.integrated_titlebar = integrated;
        app.window_controls = window_controls;
        app.system_text_scale = text_scale::system();
        // Windows' Animation effects setting, which Preferences ▸ Animation effects follows unless
        // it is set to On or Off.
        #[cfg(target_os = "windows")]
        {
            if let Some(window) = cc.winit_window() {
                set_taskbar_icon(window);
            }
            app.motion.source = Some(motion::source());
        }
        app.set_startup_update_check(true);
        app.printer_source = Some(iridium_pdf_ui_egui::system_printers());
        app.os_key_store_ids = cfg!(any(target_os = "macos", target_os = "windows"));
        #[cfg(target_os = "macos")]
        {
            // The native menu bar. Its Quit (⌘Q) closes the window rather than ending the process as
            // winit's does (#1136), so `guard_quit` asks about unsaved changes, as for the close
            // button.
            if !app.install_native_menu(&cc.egui_ctx) {
                log::warn!("couldn't install the macOS menu bar; the menus stay in the window");
            }
        }
        if let Some(connect) = &os_events {
            app.os_events = Some(connect(&cc.egui_ctx, native_wayland(cc)));
        }
        if let Some(file) = &control_file {
            let client = app.attach_control(&cc.egui_ctx);
            if let Err(message) = open_control_channel(file, iridium_pdf_ui_egui::control::serve(client)) {
                // Before any document opens, so nothing is lost; the host stops the app.
                *startup_error.borrow_mut() = Some(message);
                return Self { app };
            }
        }
        // Autosave unsaved changes; offer to recover documents a crashed session left behind.
        if let Some(dir) = iridium_pdf_ui_egui::RecoveryStore::default_dir() {
            app.enable_recovery(iridium_pdf_ui_egui::RecoveryStore::new(dir));
        }
        // The OCR Verify screen stores its per-language dictionary beside recovery data.
        app.ocr_verify.dictionary_dir = iridium_pdf_ui_egui::default_dictionary_dir();
        if let Some(state) = &cc.wgpu_render_state {
            let adapter = state.adapter.get_info();
            let max_texture_dimension_2d = state.device.limits().max_texture_dimension_2d;
            log::info!(
                "renderer: wgpu; adapter: {} ({:?}, {:?}); max_texture_dimension_2d: {max_texture_dimension_2d}",
                adapter.name,
                adapter.device_type,
                adapter.backend
            );
            notify_software_renderer(&mut app, adapter.device_type);
        } else {
            log::info!("renderer: OpenGL (glow)");
        }
        // A portable marker whose data folder can't be written (#157): say where settings went.
        if let Some(w) = &iridium_pdf_ui_egui::portable::current().unwritable {
            app.notify_fmt(
                "Portable mode is off: {folder} can't be written ({error}). Settings are kept in your user folder instead.",
                &[("folder", &w.folder.display().to_string()), ("error", &w.error)],
            );
        }
        let defer_layout = two_paged && !files.is_empty() && !create_images;
        if create_images {
            if let Err(e) = app.begin_image_import_paths(&files) {
                app.notify(e);
            }
        } else if combine {
            app.reopen_last_files(&[]);
            app.combine_paths(&files);
        } else {
            // With the preference on, last session's files come back first; files named on
            // the command line open after them, in front (#442).
            app.reopen_last_files(&files);
            startup::open_files(&mut app, &files, two_paged);
        }
        for (k, v) in options {
            let result = if defer_layout && k == "layout" { app.set_launch_layout(&v) } else { app.set_option(&k, &v) };
            if let Err(e) = result {
                log::warn!("--{k} {v}: {e}");
            }
        }
        Self { app }
    }

    /// Per-frame logic, called before `ui`.
    pub fn logic(&mut self, ctx: &egui::Context) {
        self.app.run_logic(ctx);
    }

    /// Raw input, forwarded to the app.
    pub fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        eframe::App::raw_input_hook(&mut self.app, ctx, raw);
    }

    /// Draw the app into the given region.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.app.run_ui(ui);
    }

    /// Open a file path (as a string, matching the app's existing convention).
    pub fn open_path(&mut self, path: &str) {
        self.app.open_path(path);
    }

    /// What the app does on quit: an update waiting for the exit starts. Preferences and the window
    /// go to eframe's storage through [`eframe::App::save`], which this type implements.
    pub fn save_state(&mut self) {
        eframe::App::on_exit(&mut self.app, None);
    }

    /// False when a document has unsaved changes (text typed into a field included) or an OCR
    /// review is open, so the host must ask before closing.
    pub fn can_close(&self) -> bool {
        !self.app.work_at_risk()
    }
}

/// Lets `eframe::run_native` host the shell directly; a suite host forwards `save` to it.
impl eframe::App for Shell {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        Shell::raw_input_hook(self, ctx, raw);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::App::save(&mut self.app, storage);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_state();
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Shell::logic(self, ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Shell::ui(self, ui);
    }
}

/// Tell the user when wgpu draws on the processor (WARP on Windows, llvmpipe on Linux) rather than
/// a GPU, which makes everything slower. egui-wgpu only logs it, so the reporter of #519 had to find
/// it in iridium_pdf.log. The OpenGL fallback isn't covered: glow only reports its renderer through
/// `unsafe` calls.
fn notify_software_renderer(app: &mut PdfApp, device_type: eframe::wgpu::DeviceType) {
    if device_type == eframe::wgpu::DeviceType::Cpu {
        app.notify_tr("PDF is drawing without a graphics processor, so it may be slow. Updating the graphics driver may help.");
    }
}

/// Whether the window is a native Wayland one (not X11 or XWayland).
#[cfg(all(unix, not(target_os = "macos")))]
fn native_wayland(cc: &eframe::CreationContext<'_>) -> bool {
    use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
    cc.display_handle().is_ok_and(|handle| matches!(handle.as_raw(), RawDisplayHandle::Wayland(_)))
}

#[cfg(not(all(unix, not(target_os = "macos"))))]
fn native_wayland(_: &eframe::CreationContext<'_>) -> bool {
    false
}

/// Decode `png` as a [`winit::window::Icon`] for the Windows taskbar and Alt+Tab (`ICON_BIG`).
#[cfg(target_os = "windows")]
fn taskbar_icon(png: &[u8]) -> Option<winit::window::Icon> {
    let icon = match eframe::icon_data::from_png_bytes(png) {
        Ok(icon) => icon,
        Err(e) => {
            log::warn!("taskbar icon: {e}");
            return None;
        }
    };
    match winit::window::Icon::from_rgba(icon.rgba, icon.width, icon.height) {
        Ok(icon) => Some(icon),
        Err(e) => {
            log::warn!("taskbar icon: {e}");
            None
        }
    }
}

/// Install the decoded [`winit::window::Icon`] from `png` via `set` when decoding succeeds.
#[cfg(target_os = "windows")]
fn install_taskbar_icon(png: &[u8], set: impl FnOnce(winit::window::Icon)) {
    if let Some(icon) = taskbar_icon(png) {
        set(icon);
    }
}

/// Install both window icons before the first frame. eframe's icon setter is disabled in
/// `native_options`, so it cannot replace the full-resolution taskbar/Alt+Tab icon (#3818).
#[cfg(target_os = "windows")]
fn set_taskbar_icon(window: &winit::window::Window) {
    use winit::platform::windows::{IconExtWindows as _, WindowExtWindows as _};
    install_taskbar_icon(APP_ICON_PNG, |icon| window.set_taskbar_icon(Some(icon)));
    // The title bar's icon: the small image drawn for this scale in the exe's own icon (build.rs).
    let side = (16.0 * window.scale_factor()).round().clamp(16.0, 256.0) as u32;
    match winit::window::Icon::from_resource(1, Some(winit::dpi::PhysicalSize::new(side, side))) {
        Ok(icon) => window.set_window_icon(Some(icon)),
        Err(e) => log::warn!("window icon: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_software_renderer_is_shown_to_the_user() {
        use eframe::wgpu::DeviceType;
        // Issue #519: WARP or llvmpipe is announced in the app, not just in iridium_pdf.log.
        let mut app = PdfApp::new();
        notify_software_renderer(&mut app, DeviceType::Cpu);
        let notice = app.toast.as_ref().map(|(m, _)| m.as_str()).unwrap_or_default();
        assert!(notice.contains("without a graphics processor"), "{notice:?}");
        for gpu in [DeviceType::IntegratedGpu, DeviceType::DiscreteGpu, DeviceType::VirtualGpu, DeviceType::Other] {
            let mut app = PdfApp::new();
            notify_software_renderer(&mut app, gpu);
            assert!(app.toast.is_none(), "{gpu:?}: {:?}", app.toast);
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn the_windows_taskbar_icon_installs_from_the_bundled_png() {
        let mut installed = None;
        install_taskbar_icon(APP_ICON_PNG, |icon| installed = Some(icon));
        assert!(installed.is_some(), "issue #3818: ICON_BIG loads and installs from APP_ICON_PNG");
        let mut bad = None;
        install_taskbar_icon(b"not a png", |icon| bad = Some(icon));
        assert!(bad.is_none());
    }
}
