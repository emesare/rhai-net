use rhai::plugin::*;

#[export_module]
pub mod tcp_functions {
    use crate::util::convert_to_int;
    use std::{
        cell::RefCell,
        io::{Read, Write},
        net::{SocketAddr, TcpListener, TcpStream},
        rc::Rc,
        time::Duration,
    };

    pub type SharedTcpStream = Rc<RefCell<TcpStream>>;

    pub mod listener_functions {
        pub type SharedTcpListener = Rc<RefCell<TcpListener>>;

        /// Returns a listener bound to the specified address which can then be used to accept incoming connections.
        #[rhai_fn(return_raw)]
        pub fn tcp_listen(addr: SocketAddr) -> Result<SharedTcpListener, Box<EvalAltResult>> {
            let listener = TcpListener::bind(addr).map_err(|e| e.to_string())?;
            Ok(Rc::new(RefCell::new(listener)))
        }

        /// Helper function for `tcp_listen(addr)` that takes a string instead of `SocketAddr`.
        #[rhai_fn(return_raw, name = "tcp_listen")]
        pub fn tcp_listen_str(
            ctx: NativeCallContext,
            addr_raw: ImmutableString,
        ) -> Result<SharedTcpListener, Box<EvalAltResult>> {
            let addr = ctx.call_native_fn::<SocketAddr>("addr", (addr_raw,))?;
            tcp_listen(addr)
        }

        /// Accept a new incoming connection from this listener. Returns the associated stream and the remote peer's address.
        ///
        /// Notes:
        /// - Blocks engine thread until a new connection is established.
        #[rhai_fn(global, pure, return_raw)]
        pub fn accept(
            listener: &mut SharedTcpListener,
        ) -> Result<SharedTcpStream, Box<EvalAltResult>> {
            let (stream, _) = listener.borrow_mut().accept().map_err(|e| e.to_string())?;
            Ok(Rc::new(RefCell::new(stream)))
        }
    }

    /// Returns a connection to a remote host.
    #[rhai_fn(return_raw)]
    pub fn tcp_connect(addr: SocketAddr) -> Result<SharedTcpStream, Box<EvalAltResult>> {
        let stream = TcpStream::connect(addr).map_err(|e| e.to_string())?;
        Ok(Rc::new(RefCell::new(stream)))
    }

    /// Helper function for `tcp_connect(addr)` that takes a string instead of `SocketAddr`.
    #[rhai_fn(return_raw, name = "tcp_connect")]
    pub fn tcp_connect_str(
        ctx: NativeCallContext,
        addr_raw: ImmutableString,
    ) -> Result<SharedTcpStream, Box<EvalAltResult>> {
        let addr = ctx.call_native_fn::<SocketAddr>("addr", (addr_raw,))?;
        tcp_connect(addr)
    }

    // TODO: Make `duration_ms` an actual [Duration].
    /// Returns a connection to a remote host, timing out after the specified duration (in milliseconds).
    #[rhai_fn(return_raw, name = "tcp_connect")]
    pub fn tcp_connect_with_timeout(
        addr: SocketAddr,
        duration_ms: rhai::INT,
    ) -> Result<SharedTcpStream, Box<EvalAltResult>> {
        let dur = Duration::from_millis(duration_ms.unsigned_abs());
        let stream = TcpStream::connect_timeout(&addr, dur).map_err(|e| e.to_string())?;
        Ok(Rc::new(RefCell::new(stream)))
    }

    /// Helper function for `tcp_connect(addr, timeout_duration)` that takes a string instead of `SocketAddr`.
    #[rhai_fn(return_raw, name = "tcp_connect")]
    pub fn tcp_connect_with_timeout_str(
        ctx: NativeCallContext,
        addr_raw: ImmutableString,
        duration_ms: rhai::INT,
    ) -> Result<SharedTcpStream, Box<EvalAltResult>> {
        let addr = ctx.call_native_fn::<SocketAddr>("addr", (addr_raw,))?;
        tcp_connect_with_timeout(addr, duration_ms)
    }

    /// Shuts down the stream, both read and write.
    #[rhai_fn(global, pure, return_raw)]
    pub fn shutdown(stream: &mut SharedTcpStream) -> Result<(), Box<EvalAltResult>> {
        stream
            .borrow_mut()
            .shutdown(std::net::Shutdown::Both)
            .map_err(|e| e.to_string().into())
    }

    /// Reads from the tcp stream until EOF and returns it as a string, respects the engine's `max_string_size`.
    ///
    /// Throws an exception when:
    /// - The read function encounters an I/O error, such as the connection being closed prematurely.
    #[rhai_fn(global, pure, return_raw, name = "read_string")]
    pub fn read_to_string(
        ctx: NativeCallContext,
        stream: &mut SharedTcpStream,
    ) -> Result<String, Box<EvalAltResult>> {
        read_to_string_with_len(ctx, stream, 0)
    }

    /// Reads from the tcp stream up to the passed `len` and returns it as a string, respects the engine's `max_string_size`.
    ///
    /// Notes:
    /// - Passing 0 as len will read until EOF which means that this call will block until the remote writer closes the tcp stream.
    ///
    /// Throws an exception when:
    /// - The read function encounters an I/O error, such as the connection being closed prematurely.
    #[rhai_fn(global, pure, return_raw, name = "read_string")]
    pub fn read_to_string_with_len(
        ctx: NativeCallContext,
        stream: &mut SharedTcpStream,
        len: rhai::INT,
    ) -> Result<String, Box<EvalAltResult>> {
        let mut buf: Vec<u8> = Vec::new();

        let max_len = ctx.engine().max_string_size();
        let res = match max_len {
            0 if len == 0 => stream.borrow_mut().read_to_end(&mut buf),
            0 if len > 0 => {
                buf.resize(len as usize, 0);
                stream.borrow_mut().read(&mut buf)
            }
            _ if len == 0 => {
                buf.resize(max_len, 0);
                stream.borrow_mut().read(&mut buf)
            }
            _ => {
                buf.resize(max_len.min(len as usize), 0);
                stream.borrow_mut().read(&mut buf)
            }
        };

        match res {
            Ok(read_len) => {
                buf.truncate(read_len);
                String::from_utf8(buf).map_err(|e| e.to_string().into())
            }
            Err(e) => Err(e.to_string().into()),
        }
    }

    /// Writes the string into the tcp stream.
    ///
    /// Throws an exception when:
    /// - The write function encounters an I/O error, such as the connection being closed prematurely.
    #[rhai_fn(global, pure, return_raw, name = "write")]
    pub fn write_with_string(
        stream: &mut SharedTcpStream,
        str: &str,
    ) -> Result<rhai::INT, Box<EvalAltResult>> {
        let written_len = stream
            .borrow_mut()
            .write(str.as_bytes())
            .map_err(|e| e.to_string())?;
        convert_to_int(written_len)
    }

    #[cfg(not(feature = "no_index"))]
    pub mod blob_functions {
        use rhai::Blob;

        /// Reads from the tcp stream up to the passed `len` and returns it as a `Blob`, respects the engine's `max_array_size`.
        ///
        /// Notes:
        /// - Passing 0 as len will read until EOF which means that this call will block until the remote writer closes the tcp stream.
        ///
        /// Throws an exception when:
        /// - The read function encounters an I/O error, such as the connection being closed prematurely.
        #[rhai_fn(global, pure, return_raw, name = "read_blob")]
        pub fn read_to_blob_with_len(
            ctx: NativeCallContext,
            stream: &mut SharedTcpStream,
            len: rhai::INT,
        ) -> Result<Blob, Box<EvalAltResult>> {
            let mut buf: Vec<u8> = Vec::new();

            let max_len = ctx.engine().max_array_size();
            let res = match max_len {
                0 if len == 0 => stream.borrow_mut().read_to_end(&mut buf),
                0 if len > 0 => {
                    buf.resize(len as usize, 0);
                    stream.borrow_mut().read(&mut buf)
                }
                _ if len == 0 => {
                    buf.resize(max_len, 0);
                    stream.borrow_mut().read(&mut buf)
                }
                _ => {
                    buf.resize(max_len.min(len as usize), 0);
                    stream.borrow_mut().read(&mut buf)
                }
            };

            match res {
                Ok(_) => Ok(buf),
                Err(e) => Err(e.to_string().into()),
            }
        }

        /// Reads from the tcp stream into the provided `Blob` with the read length being returned.
        ///
        /// Throws an exception when:
        /// - The read function encounters an I/O error, such as the connection being closed prematurely.
        #[rhai_fn(global, pure, return_raw)]
        pub fn read_from_tcp(
            blob: &mut Blob,
            stream: SharedTcpStream,
        ) -> Result<rhai::INT, Box<EvalAltResult>> {
            match stream.borrow_mut().read(blob) {
                Ok(len) => convert_to_int(len),
                Err(e) => Err(e.to_string().into()),
            }
        }

        /// Writes the blob into the tcp stream, returning how many bytes were written.
        ///
        /// Throws an exception when:
        /// - The write function encounters an I/O error, such as the connection being closed prematurely.
        #[rhai_fn(global, pure, return_raw)]
        pub fn write_to_tcp(
            blob: &mut Blob,
            stream: SharedTcpStream,
        ) -> Result<rhai::INT, Box<EvalAltResult>> {
            match stream.borrow_mut().write(blob) {
                Ok(len) => convert_to_int(len),
                Err(e) => Err(e.to_string().into()),
            }
        }
    }
}
