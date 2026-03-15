mod app;
mod http;

use std::{
    io,
    net::AddrParseError,
    result,
};

pub type AppResult<T> = result::Result<T, AppErr>;

pub type HttpResult<T> = result::Result<T, HttpStatus>;

#[derive(Debug)]
pub enum AppErr {
    DeserializeTOML(toml::de::Error),
    SerDeJSON(serde_json::Error),
    NonBlocking(io::Error),
    ParseAddr(AddrParseError),
    TmplNotFound(tera::Error),
    NotFound(io::Error),
    Other(io::Error),
    EmptyBuffer,
    Buffering,
    IncompleteRequest,
    TooLarge,
    NoServer,
    NoClient,
    ExtNotFound,
    NoCGI,
    Custom(String),
}

#[derive(Debug)]
pub struct HttpStatus {
    pub status_code: u16,
    pub message:     String,
}
