//! Plugin installer tool for [`bmod`](https://github.com/parasyte/bmod).
//!
//! This is usually executed as a Cargo alias defined in the plugin repo, but it can also be
//! installed with `cargo install`.

pub use crate::cli::Args;
use error_iter::ErrorIter as _;
use onlyargs::OnlyArgs;
use std::{
    env::VarError,
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, ExitCode},
};
use thiserror::Error;

mod cli;

/// All the ways in which installing a plugin can fail.
#[derive(Debug, Error)]
pub enum Error {
    /// Argument parsing errors.
    #[error("Argument parsing error")]
    Cli(#[from] onlyargs::CliError),

    /// File system I/O errors.
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    /// Cargo build errors.
    #[error("Cargo build failed with status code: {0:?}")]
    Build(Option<i32>),

    /// A required environment variable is missing.
    #[error("Missing APPDATA env var")]
    MissingEnv(#[from] VarError),
}

/// The main installer.
///
/// Use this in your executable if you want to customize how errors are reported.
pub fn install() -> Result<(), Error> {
    let appdata = std::env::var("APPDATA")?;
    let args: Args = onlyargs::parse()?;

    // Handle `--help` and `--version` options.
    if args.help {
        args.show_help_and_exit();
    } else if args.version {
        args.show_version_and_exit();
    }

    // Run `cargo build`.
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if args.release {
        cmd.arg("--release");
    };
    let status = cmd.args(["--package", &args.package]).spawn()?.wait()?;

    if !status.success() {
        return Err(Error::Build(status.code()));
    }

    // Copy the DLL to the plugin directory.
    let release = if args.release { "release" } else { "debug" };
    let plugin_name = args.package.replace('-', "_");
    let dll = format!("{}.dll", plugin_name);
    let src = PathBuf::from_iter(["target", release, &dll]);
    let dest = PathBuf::from_iter([&appdata, "bakkesmod", "bakkesmod", "plugins", &dll]);
    std::fs::copy(src, dest)?;

    // Auto-enable the plugin.
    let path = PathBuf::from_iter([
        &appdata,
        "bakkesmod",
        "bakkesmod",
        "data",
        "newfeatures.apply",
    ]);
    let mut f = File::options().create(true).append(true).open(path)?;
    f.write_all(format!("plugin load {} ; writeconfig\n", plugin_name).as_bytes())?;

    Ok(())
}

/// A wrapper for [`run`].
///
/// Intended to be used as a simple `main` function. This prints the CLI help text and the chain of
/// error messages. If you wish to customize error handling, use [`run`] instead.
pub fn run() -> ExitCode {
    match install() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            if matches!(err, Error::Cli(_)) {
                eprintln!("{}", Args::help());
            }

            eprintln!("Build error: {err}");
            for source in err.sources().skip(1) {
                eprintln!("  Caused by: {source}");
            }
            ExitCode::FAILURE
        }
    }
}
