//! `gsettings` reader for the title-bar button layout (`decorations`).

/// How long `gsettings` may take before launch goes on without it.
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(500);

/// Run `gsettings` and return what it printed, or `None` if it is missing, fails or hangs.
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub(crate) fn gsettings(args: &[&str]) -> Option<String> {
    use std::io::Read;
    use std::process::{Command, Stdio};
    let mut child = Command::new("gsettings").args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => break,
            Ok(None) if start.elapsed() < TIMEOUT => std::thread::sleep(std::time::Duration::from_millis(5)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
    let mut out = String::new();
    // A number fits in far less; a misbehaving `gsettings` can't make launch read without end.
    child.stdout.take()?.take(256).read_to_string(&mut out).ok()?;
    Some(out)
}
