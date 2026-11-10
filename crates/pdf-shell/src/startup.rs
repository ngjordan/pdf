use iridium_pdf_ui_egui::PdfApp;

pub(crate) fn open_files(app: &mut PdfApp, files: &[String], two_paged: bool) {
    for path in files {
        if two_paged {
            // The per-file override wins over initial view without changing saved defaults.
            app.open_path_with_display(path, iridium_pdf_ui_egui::canvas::PageLayout::TwoUp, false);
        } else {
            app.open_path(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_override_is_per_file_and_does_not_persist() {
        use std::io::Write as _;
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("iridium-pdf-two-paged-{}-{nonce}.pdf", std::process::id()));
        let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(&path).unwrap();
        file.write_all(include_bytes!("../../ui-egui/tests/data/form.pdf")).unwrap();
        drop(file);
        let second_path = path.with_extension("second.pdf");
        std::fs::copy(&path, &second_path).unwrap();
        let text = path.to_str().unwrap();
        let second = second_path.to_str().unwrap();
        for enabled in [false, true] {
            let mut app = PdfApp::new();
            app.set_option("default-layout", "single").unwrap();
            app.open_bytes("restored.pdf", None, include_bytes!("../../ui-egui/tests/data/form.pdf").to_vec()).unwrap();
            let restored = app.views[0].id;
            open_files(&mut app, &[text.into(), second.into()], enabled);
            assert_eq!(app.views.len(), 3);
            assert_eq!(app.views[0].id, restored);
            assert_eq!(app.views[0].layout, iridium_pdf_ui_egui::canvas::PageLayout::Single);
            for view in app.views.iter().skip(1) {
                assert_eq!(
                    view.layout,
                    if enabled { iridium_pdf_ui_egui::canvas::PageLayout::TwoUp } else { iridium_pdf_ui_egui::canvas::PageLayout::Single }
                );
                assert!(!view.continuous());
            }
            let persisted: serde_json::Value = serde_json::from_str(&app.persist()).unwrap();
            assert_eq!(persisted["default_layout"], "single");
            assert!(persisted.get("two_paged").is_none());
            // Existing --layout applies after the flag to the active document only.
            if enabled {
                app.set_launch_layout("continuous").unwrap();
            } else {
                app.set_option("layout", "continuous").unwrap();
            }
            assert_eq!(app.views[2].layout, iridium_pdf_ui_egui::canvas::PageLayout::Single);
            assert!(app.views[2].continuous());
            app.set_option("default-layout", "two-up").unwrap();
            assert_eq!(app.view_defaults.layout, iridium_pdf_ui_egui::canvas::PageLayout::TwoUp);
            assert!(app.view_defaults.continuous);
            open_files(&mut app, &[], enabled);
            assert_eq!(app.views.len(), 3);
        }
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(second_path).unwrap();
    }

    #[test]
    fn explicit_launch_layout_preserves_argument_order_and_active_file() {
        use iridium_pdf_ui_egui::canvas::PageLayout;
        let path = std::env::temp_dir().join(format!("iridium-pdf-explicit-layout-{}.pdf", std::process::id()));
        std::fs::write(&path, include_bytes!("../../ui-egui/tests/data/form.pdf")).unwrap();
        let second_path = path.with_extension("second.pdf");
        std::fs::copy(&path, &second_path).unwrap();
        let names = [path.to_str().unwrap(), second_path.to_str().unwrap()];
        for prefix in [["--two-paged", "--layout", "single"], ["--layout", "single", "--two-paged"]] {
            for count in [1, 2] {
                // What `startup::Arguments::parse` yields for these flags (tested in the app).
                let files: Vec<String> = names.iter().take(count).map(|n| (*n).to_owned()).collect();
                let two_paged = prefix.contains(&"--two-paged");
                let mut app = PdfApp::new();
                app.set_option("default-layout", "continuous").unwrap();
                app.open_bytes("restored.pdf", None, include_bytes!("../../ui-egui/tests/data/form.pdf").to_vec()).unwrap();
                open_files(&mut app, &files, two_paged);
                app.set_launch_layout("single").unwrap();
                assert_eq!(app.active, Some(count));
                assert!(app.views[0].continuous());
                for view in app.views.iter().skip(1).take(count - 1) {
                    assert_eq!(view.layout, PageLayout::TwoUp);
                }
                assert_eq!(app.views[count].layout, PageLayout::Single);
                assert!(!app.views[count].continuous());
                assert!(app.view_defaults.continuous);
            }
        }
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(second_path).unwrap();
    }
}

#[cfg(test)]
mod repeated_file_tests {
    use super::*;

    #[test]
    fn a_repeated_file_reuses_its_tab_and_receives_the_launch_override() {
        use iridium_pdf_ui_egui::canvas::PageLayout;
        let path = std::env::temp_dir().join(format!("iridium-pdf-repeated-startup-{}.pdf", std::process::id()));
        std::fs::write(&path, include_bytes!("../../ui-egui/tests/data/form.pdf")).unwrap();
        let name = path.to_str().unwrap().to_string();
        let mut app = PdfApp::new();
        app.set_option("default-layout", "continuous").unwrap();
        app.open_path(&name);
        let original = app.views[0].id;
        open_files(&mut app, &[name.clone(), name], true);
        assert_eq!(app.views.len(), 1);
        assert_eq!(app.views[0].id, original);
        assert_eq!(app.views[0].layout, PageLayout::TwoUp);
        assert!(!app.views[0].continuous());
        app.set_launch_layout("single").unwrap();
        assert_eq!(app.views[0].layout, PageLayout::Single);
        assert!(app.view_defaults.continuous);
        std::fs::remove_file(path).unwrap();
    }
}
