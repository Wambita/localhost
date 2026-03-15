mod add;
mod client;
mod core;
mod poll;
mod remove;
mod run;

#[cfg(target_os = "linux")]
use libc::epoll_event;
#[cfg(target_os = "macos")]
use libc::kevent;
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::OVERLAPPED;
use {
    crate::server::Server,
    std::{
        collections::HashMap,
        net::{
            TcpListener,
            TcpStream,
        },
        os::fd::RawFd,
    },
};

#[cfg(target_os = "linux")]
type OsEvent = epoll_event;
#[cfg(target_os = "macos")]
type OsEvent = kevent;
#[cfg(target_os = "windows")]
type OsEvent = OVERLAPPED;

#[derive(Debug)]
struct Client {
    stream:  TcpStream,
    req_buf: Vec<u8>,
}

/// Manages connection:
/// - Accepts incoming connections through TCP listeners
/// - Reading HTTP requests using non-blocking I/O
/// - Dispatches requests to the appropriate server
/// - Processes requests and sends responses back through a router system
pub struct Multiplexer {
    file_descriptor: RawFd,
    servers:         Vec<Server>,
    listeners:       Vec<TcpListener>,
    streams:         HashMap<RawFd, Client>,
    size_limit:      u64,
}
