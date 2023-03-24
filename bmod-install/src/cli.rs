use onlyargs::{CliError, OnlyArgs};
use std::ffi::OsString;

/// CLI arguments.
#[derive(Debug)]
pub struct Args {
    /// The plugin's crate name. Must be relative to the CWD.
    pub package: String,

    /// Build release profile (defaults to debug).
    pub release: bool,

    /// Show the help message and exit.
    pub help: bool,

    /// Show the application version and exit.
    pub version: bool,
}

impl OnlyArgs for Args {
    fn help() -> &'static str {
        concat!(
            env!("CARGO_PKG_NAME"),
            " v",
            env!("CARGO_PKG_VERSION"),
            "\n",
            env!("CARGO_PKG_DESCRIPTION"),
            "\n",
            "\nUsage:\n  ",
            env!("CARGO_PKG_NAME"),
            ".exe",
            " [flags] [options]\n",
            "\nFlags:\n",
            "  -p --package <name>  The plugin's crate name. Must be relative to the CWD.\n",
            "\nOptions:\n",
            "  -r --release  Build release profile (defaults to debug).\n",
            "  -h --help     Show this help message and exit.\n",
            "  --version     Show the application version and exit.\n",
        )
    }

    fn parse(args: Vec<OsString>) -> Result<Args, CliError> {
        let mut package = None;
        let mut release = false;
        let mut help = false;
        let mut version = false;

        fn missing(s: OsString) -> CliError {
            CliError::MissingValue(s.into_string().unwrap())
        }

        let mut it = args.into_iter();
        while let Some(s) = it.next() {
            match s.to_str() {
                Some("--package") | Some("-p") => {
                    let name = it
                        .next()
                        .ok_or_else(|| missing(s))?
                        .into_string()
                        .map_err(|err| CliError::ParseStrError("release".to_string(), err))?;

                    package = Some(name);
                }
                Some("--release") | Some("-r") => {
                    release = true;
                }
                Some("--help") | Some("-h") => {
                    help = true;
                }
                Some("--version") => {
                    version = true;
                }
                _ => return Err(onlyargs::CliError::Unknown(s)),
            }
        }

        // Required arguments are set to defaults if `--help` or `--version` are present.
        let package = (help || version)
            .then(String::new)
            .or(package)
            .ok_or_else(|| CliError::MissingRequired("package".to_string()))?;

        Ok(Self {
            package,
            release,
            help,
            version,
        })
    }
}
