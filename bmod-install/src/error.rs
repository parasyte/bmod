use onlyerror::Error;

/// All the ways in which installing a plugin can fail.
#[derive(Debug, Error)]
pub enum Error {
    /// Argument parsing error.
    Cli(#[from] onlyargs::CliError),

    /// File system I/O error.
    Io(#[from] std::io::Error),

    /// Cargo build errors.
    #[error("Cargo build failed with status code: {0:?}.")]
    Build(Option<i32>),

    /// A required environment variable is missing.
    MissingEnv(#[from] std::env::VarError),

    /// RCon invalid hostname.
    ///
    /// This happens when the system is misconfigured.
    #[error("RCon invalid hostname; could not resolve `localhost`.")]
    InvalidHostname,

    /// RCon TCP connection failed.
    Connection(#[source] std::io::Error),

    /// RCon WebSocket errors.
    WebSocket(#[from] ws_tool::errors::WsError),

    /// RCon authentication failed.
    Auth,
}
