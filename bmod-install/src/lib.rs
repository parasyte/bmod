//! Plugin installer tool for [`bmod`](https://github.com/parasyte/bmod).
//!
//! This is usually executed as a Cargo alias defined in the plugin repo, but it can also be
//! installed with `cargo install`.

pub use crate::cli::Args;
pub use crate::error::Error;
use error_iter::ErrorIter as _;
use log::{debug, trace};
use onlyargs::OnlyArgs;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{Duration, Instant},
};

mod cli;
mod error;
mod rcon;

/// The main installer.
///
/// Use this in your executable if you want to customize how errors are reported.
pub fn install() -> Result<(), Error> {
    let time = Instant::now();
    let args: Args = onlyargs::parse()?;

    trace!("Got args: {args:#?}");
    if args.help {
        args.show_help_and_exit();
    } else if args.version {
        args.show_version_and_exit();
    }

    let bakkesmod = match args.bakkesmod {
        Some(path) => path,
        None => {
            let appdata = std::env::var("APPDATA")?;

            PathBuf::from_iter([&appdata, "bakkesmod", "bakkesmod"])
        }
    };

    debug!("Running `cargo build`");
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if args.release {
        cmd.arg("--release");
    };
    let status = cmd.args(["--package", &args.package]).spawn()?.wait()?;

    if !status.success() {
        return Err(Error::Build(status.code()));
    }

    eprintln!("Installing {}", args.package);

    let plugin_name = args.package.replace('-', "_");
    install_plugin(&plugin_name, bakkesmod, args.release)?;

    eprintln!("Finished install in {:.2?}", time.elapsed());

    Ok(())
}

fn install_plugin(plugin_name: &str, bakkesmod: PathBuf, release: bool) -> Result<(), Error> {
    if let Ok(password) = get_rcon_password(&bakkesmod) {
        debug!("Connecting to RCon.");
        if let Some(mut client) = rcon::RCon::new(&password)? {
            debug!("Rocket League is running. Using RCon to reload the plugin.");
            client.plugin_unload(plugin_name)?;

            debug!("Waiting for bakkesmod to close the old plugin.");
            std::thread::sleep(Duration::from_secs(1));

            copy_plugin(plugin_name, &bakkesmod, release)?;
            client.plugin_load(plugin_name)?;

            return Ok(());
        }

        debug!("Timed out waiting for RCon.");
    }

    copy_plugin(plugin_name, &bakkesmod, release)?;
    deferred_enable_plugin(plugin_name, bakkesmod)?;

    Ok(())
}

fn copy_plugin(plugin_name: &str, bakkesmod: &Path, release: bool) -> Result<(), Error> {
    debug!("Copying the DLL to the plugin directory.");

    let release = if release { "release" } else { "debug" };
    let dll = format!("{}.dll", plugin_name);
    let src = PathBuf::from_iter(["target", release, &dll]);
    let dest = bakkesmod.join("plugins").join(dll);
    std::fs::copy(src, dest)?;

    Ok(())
}

fn deferred_enable_plugin(plugin_name: &str, bakkesmod: PathBuf) -> Result<(), Error> {
    debug!("Deferring plugin install to next bakkesmod launch.");

    let path = bakkesmod.join("data").join("newfeatures.apply");
    let mut f = File::options().create(true).append(true).open(path)?;
    f.write_all(format!("plugin load {} ; writeconfig\n", plugin_name).as_bytes())?;

    Ok(())
}

fn get_rcon_password(bakkesmod: &Path) -> Result<String, Error> {
    debug!("Looking up RCon password.");

    let path = bakkesmod.join("cfg").join("config.cfg");
    let config = std::fs::read_to_string(path)?;

    debug!("Parsing bakkesmod config.");
    for line in config.lines() {
        if let Some(line) = line.strip_prefix("rcon_password ") {
            let password = line
                .split('"')
                .nth(1)
                .map_or_else(String::new, |s| s.to_string());

            return Ok(password);
        }
    }

    Ok(String::new())
}

/// A wrapper for [`run`].
///
/// Intended to be used as a simple `main` function. This prints the CLI help text and the chain of
/// error messages. If you wish to customize error handling, use [`run`] instead.
pub fn run() -> ExitCode {
    env_logger::init();

    match install() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            if matches!(err, Error::Cli(_)) {
                eprintln!("{}", Args::help());
            }

            eprintln!("Install error: {err}");
            for source in err.sources().skip(1) {
                eprintln!("  Caused by: {source}");
            }
            ExitCode::FAILURE
        }
    }
}
