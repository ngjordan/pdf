//! The UI control channel's endpoint file (`--control`).

/// Publish a started control channel in `file`.
///
/// A failure stops the app. `--control` was asked for, so carrying on without it would leave a
/// script driving nothing, and the file that couldn't be replaced may be another user's, planted
/// at a shared path such as `/tmp/pc.json` to receive the commands. This runs before any document
/// opens, so nothing is lost.
pub(crate) fn open_control_channel(file: &str, endpoint: std::io::Result<iridium_pdf_ui_egui::control::Endpoint>) -> Result<(), String> {
    match endpoint.and_then(|ep| write_control_file(file, ep.port, &ep.token).map(|()| ep.port)) {
        // Never the token (AGENTS.md §3): it stays in the owner-only file.
        Ok(port) => {
            log::info!("UI control channel on 127.0.0.1:{port} (connection details in {file})");
            Ok(())
        }
        Err(e) => {
            // cmd.exe doesn't expand `~` (#3922).
            let example = if cfg!(windows) { r"%USERPROFILE%\.iridium-pdf-control.json" } else { "~/.iridium-pdf-control.json" };
            let message = format!("--control {file}: {e}. Use a control file in a folder only you can write, such as {example}");
            log::error!("{message}");
            Err(message)
        }
    }
}

/// Write the control endpoint so that only the current user can read the token.
///
/// The JSON goes to a new file, created fresh and owner-only, which then replaces `path`. On Unix
/// the file sits next to `path` with mode 0600. On Windows it sits in a new folder next to `path`
/// whose access list, of this user and SYSTEM only, it inherits when it is created, so no other
/// account can open it at any time (#3922; review on #3956). Opening `path` itself would follow a
/// link planted there and truncate whatever it points at; a rename replaces the link and leaves
/// its target alone. A reader polling for the file also never sees it half-written.
fn write_control_file(path: &str, port: u16, token: &str) -> std::io::Result<()> {
    use std::io::ErrorKind;
    let json = serde_json::json!({ "port": port, "token": token, "pid": std::process::id() }).to_string();
    let path = std::path::Path::new(path);
    let name = path.file_name().ok_or_else(|| std::io::Error::new(ErrorKind::InvalidInput, "not a file name"))?;
    // Windows: the user's SID, read once for the private folder and for the file in it, since each
    // lookup starts `whoami` (review on #3956).
    #[cfg(windows)]
    let me = iridium_pdf_platform::owner_only::current_user_sid()?;
    #[cfg(windows)]
    let restrict = |file: &std::path::Path| iridium_pdf_platform::owner_only::restrict_to_owner(file, &me);
    #[cfg(not(windows))]
    let restrict = |_: &std::path::Path| Ok::<(), std::io::Error>(());
    // Unpredictable suffixes, so the name can't be planted in advance. `RandomState` is keyed from
    // the operating system's random source.
    let random = std::hash::RandomState::new();
    for i in 0..16u32 {
        let mut staged = std::ffi::OsString::from(".");
        staged.push(name);
        staged.push(format!(".{:016x}.tmp", std::hash::BuildHasher::hash_one(&random, i)));
        let staged = path.with_file_name(staged);
        // Windows: a file created next to `path` would inherit that folder's access list, which may
        // let other users read it, and restricting it afterwards leaves a window in which one of
        // them opens a handle; Windows checks access only when a handle is opened, so a later
        // change of the list doesn't revoke it. The file is created in a private folder instead,
        // and that folder is removed once the file has left it.
        #[cfg(windows)]
        let (staging, staged) = {
            match iridium_pdf_platform::owner_only::create_private_dir(&staged, &me) {
                Ok(()) => {}
                Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
                // As for a file below: something at the name we may not open still takes it.
                Err(e) if e.kind() == ErrorKind::PermissionDenied && staged.symlink_metadata().is_ok() => continue,
                Err(e) => return Err(e),
            }
            let file = staged.join(name);
            (staged, file)
        };
        let done = stage_control_file(&staged, path, &json, &restrict);
        #[cfg(windows)]
        let _ = std::fs::remove_dir(&staging);
        match done {
            Some(done) => return done,
            None => continue,
        }
    }
    Err(std::io::Error::new(ErrorKind::AlreadyExists, "no free name for the control file's temporary copy"))
}

/// Create `staged` fresh and owner-only, `restrict` it, write `json` to it and rename it to `path`.
/// `None` when something already has the name, so the caller tries another. On any other failure
/// the new file is removed and nothing is left at `path`'s side.
fn stage_control_file(
    staged: &std::path::Path,
    path: &std::path::Path,
    json: &str,
    restrict: &dyn Fn(&std::path::Path) -> std::io::Result<()>,
) -> Option<std::io::Result<()>> {
    use std::io::{ErrorKind, Write};
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let file = match opts.open(staged) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => return None,
        // Windows refuses `create_new` on a folder with "access denied"; the name is taken all
        // the same. A folder we can't write to, with nothing at the name, still fails here.
        Err(e) if e.kind() == ErrorKind::PermissionDenied && staged.symlink_metadata().is_ok() => return None,
        Err(e) => return Some(Err(e)),
    };
    // Windows: the file inherited the private folder's list. It also gets a list of its own, so it
    // stays private wherever the rename puts it, and that is checked before the token is written;
    // nothing is written if that fails (#3922). Unix has mode 0600 above, and `restrict` does nothing.
    if let Err(e) = restrict(staged) {
        drop(file);
        let _ = std::fs::remove_file(staged);
        return Some(Err(e));
    }
    // The block closes the file before it is renamed.
    let written = {
        let mut file = file;
        file.write_all(json.as_bytes())
    };
    let done = written.and_then(|()| std::fs::rename(staged, path));
    if done.is_err() {
        let _ = std::fs::remove_file(staged);
    }
    Some(done)
}

#[cfg(test)]
mod tests {
    use super::open_control_channel;
    use iridium_pdf_ui_egui::control::Endpoint;

    fn endpoint() -> std::io::Result<Endpoint> {
        Ok(Endpoint { port: 4321, token: "0123456789abcdef".into() })
    }

    #[test]
    fn a_control_channel_that_cant_be_published_stops_the_app() {
        let dir = std::env::temp_dir().join(format!("iridium-pdf-app-control-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = |name: &str| dir.join(name).to_str().unwrap().to_owned();

        // A folder in the way stands in for a file another user created first: the write fails.
        std::fs::create_dir(dir.join("taken.json")).unwrap();
        let err = open_control_channel(&path("taken.json"), endpoint()).unwrap_err();
        assert!(err.contains("--control") && err.contains("taken.json") && err.contains("only you can write"), "{err}");
        // No listener: nothing to publish.
        let err = open_control_channel(&path("pc.json"), Err(std::io::Error::other("no loopback"))).unwrap_err();
        assert!(err.contains("no loopback"), "{err}");
        assert!(!dir.join("pc.json").exists());

        open_control_channel(&path("pc.json"), endpoint()).unwrap();
        let written: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.join("pc.json")).unwrap()).unwrap();
        assert_eq!((written["port"].as_u64(), written["token"].as_str()), (Some(4321), Some("0123456789abcdef")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A fresh folder per call: tests run in parallel.
    fn scratch() -> std::path::PathBuf {
        static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let d = std::env::temp_dir().join(format!("iridium-pdf-control-file-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// The control file's port and token, and every name in its folder.
    fn written(dir: &std::path::Path) -> (serde_json::Value, Vec<String>) {
        let json = serde_json::from_str(&std::fs::read_to_string(dir.join("ctl.json")).unwrap()).unwrap();
        let mut names: Vec<String> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        (json, names)
    }

    #[test]
    fn the_control_file_replaces_a_hard_link_instead_of_writing_through_it() {
        // The file used to be opened with create + truncate, so a link planted at the path had its
        // target truncated and overwritten with the token.
        let dir = scratch();
        std::fs::write(dir.join("victim.txt"), "keep me").unwrap();
        std::fs::hard_link(dir.join("victim.txt"), dir.join("ctl.json")).unwrap();
        super::write_control_file(&dir.join("ctl.json").to_string_lossy(), 4242, "t0ken").unwrap();
        assert_eq!(std::fs::read_to_string(dir.join("victim.txt")).unwrap(), "keep me");
        let (json, names) = written(&dir);
        assert_eq!((json["port"].as_u64(), json["token"].as_str()), (Some(4242), Some("t0ken")));
        assert_eq!(names, ["ctl.json", "victim.txt"], "no temporary file left behind");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_control_file_replaces_a_symlink_instead_of_writing_through_it() {
        let dir = scratch();
        let (link, target) = (dir.join("ctl.json"), dir.join("victim.txt"));
        std::fs::write(&target, "keep me").unwrap();
        #[cfg(unix)]
        let made = std::os::unix::fs::symlink(&target, &link);
        #[cfg(windows)]
        let made = std::os::windows::fs::symlink_file(&target, &link);
        if let Err(e) = made {
            // Windows needs Developer Mode (or admin) for symlinks.
            eprintln!("skipped: can't create a symlink here: {e}");
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }
        super::write_control_file(&link.to_string_lossy(), 4242, "t0ken").unwrap();
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "keep me");
        assert!(!std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
        assert_eq!(written(&dir).0["token"].as_str(), Some("t0ken"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_control_file_replaces_the_one_a_previous_run_left() {
        let dir = scratch();
        std::fs::write(dir.join("ctl.json"), r#"{"port":1,"token":"old"}"#).unwrap();
        super::write_control_file(&dir.join("ctl.json").to_string_lossy(), 4242, "t0ken").unwrap();
        let (json, names) = written(&dir);
        assert_eq!(json["token"].as_str(), Some("t0ken"));
        assert_eq!(names, ["ctl.json"]);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(dir.join("ctl.json")).unwrap().permissions().mode();
            assert_eq!(mode & 0o077, 0, "only the owner can read the token: {mode:o}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// #3922: a control file created in a folder that lets every user read what's in it is still
    /// readable by its user and SYSTEM only, like mode 0600 on Unix.
    #[cfg(windows)]
    #[test]
    fn the_control_file_is_owner_only_in_a_folder_other_users_can_read() {
        use iridium_pdf_platform::owner_only;
        let dir = scratch();
        let users = std::process::Command::new("icacls").arg(&dir).arg("/grant").arg("*S-1-5-32-545:(OI)(CI)R").output().unwrap();
        assert!(users.status.success());
        // What the folder hands down, for comparison: a plain file there is readable by Users.
        std::fs::write(dir.join("plain.txt"), "x").unwrap();
        let me = owner_only::current_user_sid().unwrap();
        assert!(owner_only::foreign_access(&owner_only::sddl_of(&dir.join("plain.txt")).unwrap(), &me).is_some());
        super::write_control_file(&dir.join("ctl.json").to_string_lossy(), 4242, "t0ken").unwrap();
        let sddl = owner_only::sddl_of(&dir.join("ctl.json")).unwrap();
        assert_eq!(owner_only::foreign_access(&sddl, &me), None, "{sddl}");
        let (json, names) = written(&dir);
        assert_eq!(json["token"].as_str(), Some("t0ken"));
        assert_eq!(names, ["ctl.json", "plain.txt"], "no private staging folder left behind");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Review on #3956: writing the control file reads the user's SID once, for both the private
    /// folder and the file in it, rather than starting `whoami` for each.
    #[cfg(windows)]
    #[test]
    fn writing_the_control_file_looks_up_the_users_sid_once() {
        use iridium_pdf_platform::owner_only::sid_lookups_on_this_thread;
        let dir = scratch();
        let before = sid_lookups_on_this_thread();
        super::write_control_file(&dir.join("ctl.json").to_string_lossy(), 4242, "t0ken").unwrap();
        assert_eq!(sid_lookups_on_this_thread().saturating_sub(before), 1, "whoami runs once per control file");
        assert_eq!(written(&dir).0["token"].as_str(), Some("t0ken"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
