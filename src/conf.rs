//! `Config` is a simple struct that holds any server configuration details 
//! such as bind address and ssl information. Debug mode should produce error messages
//! in a method similar to Django error pages.

use std::io::net::ip::SocketAddr;

/// Defines the configuration of 
/// `debug` - At some point, I hope to use this to switch error handling to provide
/// useful error handling to the developer and then hiding details from a client
pub struct Config {
    /// Currently does nothing, but I want it to eventually switch whether errors are rendered
    /// to the page or whether it should send an email to the web master
    pub debug : bool,
    /// Rust is moving away from the whole SocketAddr thing. TODO change this as well
    /// See https://github.com/chris-morgan/rust-http/pull/87 
    pub bind_addr : SocketAddr,
}