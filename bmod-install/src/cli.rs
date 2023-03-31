use onlyargs::{extensions::*, CliError, OnlyArgs};
use std::{ffi::OsString, path::PathBuf};

/// CLI arguments.
#[derive(Debug)]
pub struct Args {
    /// The plugin's crate name. Must be relative to the CWD.
    pub package: String,

    /// User's local bakkesmod directory.
    pub bakkesmod: Option<PathBuf>,

    /// Build release profile (defaults to debug).
    pub release: bool,
}

impl OnlyArgs for Args {
    const HELP: &'static str = concat!(
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
        "  -h --help     Show this help message and exit.\n",
        "  -V --version  Show the application version and exit.\n",
        "  -r --release  Build release profile (defaults to debug).\n",
        "\nOptions:\n",
        "  -p --package <name>    The plugin's crate name. Must be relative to the CWD.\n",
        "  -b --bakkesmod [path]  Path for local bakkesmod directory.\n",
        "                         Default: `%AppData%\\bakkesmod\\bakkesmod`\n",
    );

    fn parse(args: Vec<OsString>) -> Result<Args, CliError> {
        let mut package = None;
        let mut bakkesmod = None;
        let mut release = false;

        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.to_str() {
                Some(name @ "--package") | Some(name @ "-p") => {
                    package = Some(args.next().parse_str(name)?);
                }
                Some(name @ "--bakkesmod") | Some(name @ "-b") => {
                    bakkesmod = Some(args.next().parse_path(name)?);
                }
                Some("--release") | Some("-r") => {
                    release = true;
                }
                Some("--help") | Some("-h") => {
                    Self::help();
                }
                Some("--version") | Some("-V") => {
                    Self::version();
                }
                _ => return Err(onlyargs::CliError::Unknown(arg)),
            }
        }

        Ok(Self {
            package: package.required("--package")?,
            bakkesmod,
            release,
        })
    }
}
