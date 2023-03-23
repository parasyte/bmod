use crate::cli::Args;
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

#[derive(Debug, Error)]
enum Error {
    #[error("Argument parsing error")]
    Cli(#[from] onlyargs::CliError),

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Cargo build failed with status code: {0:?}")]
    Build(Option<i32>),

    #[error("Missing APPDATA env var")]
    MissingEnv(#[from] VarError),
}

fn run() -> Result<(), Error> {
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
    let status = cmd.args(["--package", &args.pkg_name]).spawn()?.wait()?;

    if !status.success() {
        return Err(Error::Build(status.code()));
    }

    // Copy the DLL to the plugin directory.
    let release = if args.release { "release" } else { "debug" };
    let plugin_name = args.pkg_name.replace('-', "_");
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

fn main() -> ExitCode {
    match run() {
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
