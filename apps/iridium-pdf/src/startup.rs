//! Native launch arguments and per-file view overrides (#1354).

#[derive(Default)]
pub(super) struct Arguments {
    pub files: Vec<String>,
    pub options: Vec<(String, String)>,
    pub control_file: Option<String>,
    pub create_images: bool,
    /// `--combine`: the files go to the Combine files tab.
    pub combine: bool,
    pub new_window: bool,
    pub two_paged: bool,
    pub version: bool,
    pub help: bool,
}

impl Arguments {
    pub fn may_hand_off(&self) -> bool {
        !self.new_window && !self.files.is_empty() && self.options.is_empty() && self.control_file.is_none() && !self.create_images && !self.two_paged
    }

    pub fn parse(args: impl IntoIterator<Item = String>) -> Self {
        let mut parsed = Self::default();
        let mut args = args.into_iter();
        while let Some(a) = args.next() {
            match a.as_str() {
                "--version" => {
                    parsed.version = true;
                    break;
                }
                "-h" | "--help" => {
                    parsed.help = true;
                    break;
                }
                "--control" => parsed.control_file = args.next(),
                "--create-images" => parsed.create_images = true,
                // File Explorer's "Combine files in PDF" (#585): the files go to the Combine
                // files tab, of the running PDF when there is one.
                "--combine" => parsed.combine = true,
                "--new-window" => parsed.new_window = true,
                "--two-paged" => parsed.two_paged = true,
                flag if flag.starts_with("--two-paged=") => {
                    // It has no value: do not consume the following filename on malformed use.
                    log::warn!("{flag}: use --two-paged without a value");
                }
                flag if flag.starts_with("--") => {
                    let value = args.next().unwrap_or_default();
                    parsed.options.push((flag.trim_start_matches("--").to_string(), value));
                }
                _ => parsed.files.push(super::path_from_arg(a)),
            }
        }
        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Arguments {
        Arguments::parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn standalone_flag_keeps_files_in_either_position() {
        for args in
            [["--two-paged", "first.pdf", "second.pdf"], ["first.pdf", "--two-paged", "second.pdf"], ["first.pdf", "second.pdf", "--two-paged"]]
        {
            let parsed = parse(&args);
            assert!(parsed.two_paged);
            assert_eq!(parsed.files, ["first.pdf", "second.pdf"]);
            assert!(parsed.options.is_empty());
            assert!(!parsed.may_hand_off());
        }
        let empty = parse(&["--two-paged"]);
        assert!(empty.two_paged && empty.files.is_empty());
        let normal = parse(&["first.pdf", "second.pdf"]);
        assert!(!normal.two_paged);
        assert!(normal.may_hand_off());
        for args in [
            &["--new-window", "first.pdf"][..],
            &["--layout", "two-page", "first.pdf"],
            &["--control", "control.json", "first.pdf"],
            &["--create-images", "first.png"],
        ] {
            assert!(!parse(args).may_hand_off());
        }
    }

    #[test]
    fn existing_layout_options_are_preserved_in_either_order() {
        for args in [["--two-paged", "--layout", "single", "file.pdf"], ["--layout", "single", "--two-paged", "file.pdf"]] {
            let parsed = parse(&args);
            assert!(parsed.two_paged);
            assert_eq!(parsed.files, ["file.pdf"]);
            assert_eq!(parsed.options, [("layout".into(), "single".into())]);
        }
        let parsed = parse(&["--two-paged", "--default-layout", "two-up", "file.pdf"]);
        assert_eq!(parsed.options, [("default-layout".into(), "two-up".into())]);
    }

    #[test]
    fn newer_options_and_version_are_preserved() {
        let parsed = parse(&["--two-paged", "--new-window", "--grid-zoom", "150", "--default-mode", "read", "file:///tmp/a%20b.pdf"]);
        assert!(parsed.two_paged && parsed.new_window);
        assert_eq!(parsed.files, ["/tmp/a b.pdf"]);
        assert_eq!(parsed.options, [("grid-zoom".into(), "150".into()), ("default-mode".into(), "read".into())]);
        assert!(!parsed.may_hand_off());
        let version = parse(&["--two-paged", "--version", "ignored.pdf"]);
        assert!(version.version && version.files.is_empty());
    }

    #[test]
    fn help_keeps_the_headless_early_exit() {
        for flag in ["-h", "--help"] {
            let args = parse(&["--two-paged", flag, "ignored.pdf"]);
            assert!(args.help && args.files.is_empty());
        }
    }

    #[test]
    fn malformed_boolean_does_not_eat_a_file_or_option() {
        let args = parse(&["--two-paged=false", "first.pdf", "--layout", "single", "second.pdf"]);
        assert!(!args.two_paged);
        assert_eq!(args.files, ["first.pdf", "second.pdf"]);
        assert_eq!(args.options, [("layout".into(), "single".into())]);
    }
}
