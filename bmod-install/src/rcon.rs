use crate::error::Error;
use log::debug;
use std::{
    net::{TcpStream, ToSocketAddrs as _},
    time::Duration,
};
use ws_tool::{
    codec::StringCodec,
    connector::{get_host, get_scheme},
    http::Uri,
    ClientBuilder,
};

/// Remote Console client for bakkesmod.
pub struct RCon {
    client: StringCodec<TcpStream>,
}

impl RCon {
    /// Returns `Some(client)` if connection and authentication was successful.
    /// Returns `None` if there was a connection timeout (Rocket League needs to be running).
    pub fn new(password: &str) -> Result<Option<Self>, Error> {
        let uri = "ws://localhost:9002";
        debug!("Connecting to {uri}.");
        let uri = Uri::from_static(uri);
        let stream = match tcp_connect(&uri)? {
            Some(stream) => stream,
            None => return Ok(None),
        };
        let mut client = ClientBuilder::new().with_stream(uri, stream, StringCodec::check_fn)?;

        debug!("Authenticating.");
        client.send(format!("rcon_password {password}"))?;
        match client.receive()?.into().as_str() {
            "authyes" => Ok(Some(Self { client })),
            _ => Err(Error::Auth),
        }
    }

    /// Send `plugin load` command.
    pub fn plugin_load(&mut self, plugin: &str) -> Result<(), Error> {
        self.send(format!("plugin load {plugin};"))
    }

    /// Send `plugin unload` command.
    pub fn plugin_unload(&mut self, plugin: &str) -> Result<(), Error> {
        self.send(format!("plugin unload {plugin};"))
    }

    fn send<S: Into<String>>(&mut self, cmd: S) -> Result<(), Error> {
        let cmd = cmd.into();
        debug!("Sending command: {cmd}");
        self.client.send(cmd)?;

        Ok(())
    }
}

fn tcp_connect(uri: &Uri) -> Result<Option<TcpStream>, Error> {
    let mode = get_scheme(uri)?;
    let host = get_host(uri)?;
    let port = uri.port_u16().unwrap_or_else(|| mode.default_port());
    let addr = format!("{host}:{port}")
        .to_socket_addrs()?
        .next()
        .ok_or(Error::InvalidHostname)?;

    let timeout = Duration::from_secs(1);

    match TcpStream::connect_timeout(&addr, timeout) {
        Err(err) if err.kind() == std::io::ErrorKind::TimedOut => Ok(None),
        Err(err) => Err(Error::Connection(err)),
        Ok(stream) => Ok(Some(stream)),
    }
}
