use {
    super::Client,
    crate::{
        debug,
        utils::AppResult,
    },
    std::{
        io::{
            ErrorKind,
            Read,
        },
        net::TcpStream,
    },
};

impl Client {
    pub fn connect(stream: TcpStream) -> Self {
        Self {
            stream,
            req_buf: Vec::new(),
        }
    }

    pub(super) fn read(&mut self) -> AppResult<()> {
        let mut buf = [0u8; 32768];

        match self.stream.read(&mut buf) {
            Ok(bytes) if bytes > 0 => {
                self.req_buf
                    .extend_from_slice(&buf[..bytes]);
                Ok(())
            }
            Err(e) if e.kind() != ErrorKind::WouldBlock => Err(debug!(e).into()),
            _ => Ok(()),
        }
    }
}
