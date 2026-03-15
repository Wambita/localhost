use {
    super::{
        Client,
        Multiplexer,
    },
    crate::{
        debug,
        message::{
            Request,
            Response,
        },
        utils::{
            AppErr,
            HttpStatus,
        },
    },
    std::{
        io::Write,
        net::Shutdown,
        os::fd::{
            AsRawFd,
            RawFd,
        },
    },
};

impl Multiplexer {
    /// Starts the main process by setting a vector of potentially
    /// uninitialized events with a specified capacity. Then gets the file
    /// descriptor from the event through the number of found descriptors
    /// (nfds), finds the listener associated with the file descriptor,
    /// gets the stream and address from the associated listener and makes
    /// the stream asynchronous. Then from the stream buffer, gets
    /// the request, adds the stream file descriptor to the
    /// `Multiplexer`and finally sends the `Request` to the `Router`.
    pub fn run(&mut self) {
        let mut events = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = match self.poll(&mut events) {
                Ok(nfds) => nfds as usize,
                Err(e) => {
                    debug!(e);
                    continue;
                }
            };

            for event in events.iter().take(nfds) {
                let event = unsafe { event.assume_init() };

                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;
                #[cfg(target_os = "windows")]
                let event_fd = event.fd as RawFd;

                match self.find_listener(event_fd) {
                    Some(listener) => {
                        let (stream, _addr) = match listener.accept() {
                            Ok((stream, addr)) => (stream, addr),
                            Err(e) => {
                                debug!(e);
                                continue;
                            }
                        };

                        if let Err(e) = stream.set_nonblocking(true) {
                            debug!(e);
                            if let Err(e) = stream.shutdown(Shutdown::Both) {
                                debug!(e);
                            };
                            continue;
                        };

                        let stream_fd = stream.as_raw_fd();
                        let client = Client::connect(stream);
                        self.streams
                            .insert(stream_fd, client);

                        if let Err(e) = self.add(stream_fd) {
                            debug!(e);
                            if let Err(e) = self.remove(stream_fd) {
                                debug!(e);
                            };
                        };
                    }
                    None => {
                        if !self.can_read(&event) {
                            continue;
                        }

                        let client = match self
                            .streams
                            .get_mut(&event_fd)
                        {
                            Some(client) => client,
                            None => continue,
                        };

                        if client.read().is_err() {
                            continue;
                        }

                        let state = Request::get(&client.req_buf, self.size_limit);

                        // client.req_buf = vec![];

                        let response: Vec<u8> = match state {
                            Err(e) => match e {
                                AppErr::TooLarge => Response::from(HttpStatus::from(413)),
                                _ => continue,
                            },
                            Ok(request) => match Self::find_server(&self.servers, request.host()) {
                                Some(server) => {
                                    let addr = client
                                        .stream
                                        .local_addr()
                                        .unwrap()
                                        .to_string();
                                    server
                                        .direct(&request, &mut client.stream, &addr)
                                        .into()
                                }
                                None => Response::from(HttpStatus::from(400)),
                            },
                        }
                        .into();

                        let _ = client
                            .stream
                            .write(response.as_slice());
                        let _ = client.stream.flush();
                        let _ = self.remove(event_fd);
                    }
                }
            }
        }
    }
}
