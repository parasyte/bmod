use onlyargs::{CliError, OnlyArgs};
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

    /// Show the help message and exit.
    pub help: bool,

    /// Show the application version and exit.
    pub version: bool,
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
        "  -r --release  Build release profile (defaults to debug).\n",
        "  -h --help     Show this help message and exit.\n",
        "  -V --version  Show the application version and exit.\n",
        "                         Default: `%AppData%\\bakkesmod\\bakkesmod`\n",
        "\nOptions:\n",
        "  -p --package <name>    The plugin's crate name. Must be relative to the CWD.\n",
        "  -b --bakkesmod [path]  Path for local bakkesmod directory.\n",
    );

    fn parse(args: Vec<OsString>) -> Result<Args, CliError> {
        let mut package = None;
        let mut bakkesmod = None;
        let mut release = false;
        let mut help = false;
        let mut version = false;

        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.to_str() {
                Some(name @ "--package") | Some(name @ "-p") => {
                    package = Some(onlyargs::parse_str(name, it.next())?);
                }
                Some(name @ "--bakkesmod") | Some(name @ "-b") => {
                    bakkesmod = Some(onlyargs::parse_path(name, it.next())?);
                }
                Some("--release") | Some("-r") => {
                    release = true;
                }
                Some("--help") | Some("-h") => {
                    help = true;
                }
                Some("--version") | Some("-V") => {
                    version = true;
                }
                _ => return Err(onlyargs::CliError::Unknown(arg)),
            }
        }

        // Required arguments are set to defaults if `--help` or `--version` are present.
        let package = onlyargs::unwrap_required(help || version, "--package", package)?;

        Ok(Self {
            package,
            bakkesmod,
            release,
            help,
            version,
        })
    }
}
