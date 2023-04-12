use onlyargs_derive::OnlyArgs;

/// CLI arguments.
#[derive(Debug, OnlyArgs)]
pub struct Args {
    /// The plugin's crate name. Must be relative to the CWD.
    pub package: String,

    /// User's local bakkesmod directory.
    pub bakkesmod: Option<std::path::PathBuf>,

    /// Build release profile (defaults to debug).
    pub release: bool,
}
