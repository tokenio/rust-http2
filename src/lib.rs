//! Asynchnous HTTP/2 client and server implementation.
//!
//! Based on futures/tokio.

#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;
extern crate futures_cpupool;

extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_timer;


extern crate tls_api;
extern crate tls_api_stub;
extern crate tokio_tls_api;

extern crate void;
extern crate net2;
extern crate bytes;

mod solicit;

mod error;
mod result;
mod result_or_eof;

mod codec;
mod service;
mod service_paths;
mod client;
mod socket;
mod socket_tcp;
mod server;

#[cfg(unix)]
extern crate tokio_uds;
#[cfg(unix)]
mod socket_unix;

mod ascii;

mod common;
mod client_died_error_holder;

mod data_or_trailers;
mod data_or_headers;
mod data_or_headers_with_flag;
mod message;

mod futures_misc;

mod req_resp;
mod headers_place;

mod assert_types;

mod hpack;
mod solicit_async;
mod solicit_misc;

mod misc;

mod resp;

mod exec;

pub use socket::AnySocketAddr;

pub use solicit::HttpScheme;
pub use solicit::header::Header;
pub use solicit::header::Headers;

pub use service::Service;
pub use service_paths::ServicePaths;

pub use exec::CpuPoolOption;

pub use client::Client;
pub use client::ClientBuilder;
pub use client::client_conf::ClientConf;
pub use client::client_tls::ClientTlsOption;

pub use server::Server;
pub use server::ServerBuilder;
pub use server::server_conf::ServerConf;
pub use server::server_conf::ServerAlpn;
pub use server::server_tls::ServerTlsOption;

pub use data_or_trailers::DataOrTrailers;
pub use data_or_trailers::HttpStreamAfterHeaders;
pub use resp::Response;

pub use message::SimpleHttpMessage;

pub use error::Error;
pub use error::ErrorCode;
pub use result::Result;

/// Functions used in tests
#[doc(hidden)]
pub mod for_test {
    pub use common::ConnStateSnapshot;
    pub use server::server_conn::ServerConn;
    pub use solicit_async::recv_raw_frame_sync;

    pub use solicit::WindowSize;
    pub use solicit::DEFAULT_SETTINGS;
    pub use solicit::frame::settings::HttpSettings;

    pub mod solicit {
        pub use ::solicit::*;
    }
    pub mod hpack {
        pub use hpack::*;
    }
}
