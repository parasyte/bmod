use onlyargs::{CliError, OnlyArgs};
use std::ffi::OsString;

#[derive(Debug)]
pub struct Args {
    pub pkg_name: String,
    pub release: bool,
    pub help: bool,
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
            env!("CARGO_BIN_NAME"),
            " [flags] [options]\n",
            "\nFlags:\n",
            "  -p --package <name>  The plugin's crate name. Must be relative to the CWD.\n",
            "\nOptions:\n",
            "  -r --release  Build release profile (defaults to debug).\n",
            "  -h --help     Show this help message.\n",
            "  --version     Show the application version.\n",
        )
    }

    fn parse(args: Vec<OsString>) -> Result<Args, CliError> {
        let mut pkg_name = None;
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

                    pkg_name = Some(name);
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

        // Required arguments.
        let pkg_name = pkg_name.ok_or_else(|| CliError::MissingRequired("package".to_string()))?;

        Ok(Self {
            pkg_name,
            release,
            help,
            version,
        })
    }
}
