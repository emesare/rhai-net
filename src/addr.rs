use rhai::plugin::*;

#[export_module]
pub mod addr_functions {
    use crate::util::convert_to_int;
    use std::net::{IpAddr, SocketAddr};

    /// Creates a socket address from the passed string.
    #[rhai_fn(global, return_raw)]
    pub fn addr(raw: &str) -> Result<SocketAddr, Box<EvalAltResult>> {
        raw.parse::<SocketAddr>().map_err(|e| e.to_string().into())
    }

    /// Creates a socket address from the passed ip address string and port.
    #[rhai_fn(global, return_raw, name = "addr")]
    pub fn addr_with_port(raw: &str, port: rhai::INT) -> Result<SocketAddr, Box<EvalAltResult>> {
        match raw.parse::<IpAddr>() {
            Ok(ip) => Ok(SocketAddr::new(ip, port as u16)),
            Err(e) => Err(e.to_string().into()),
        }
    }

    /// Returns true if the socket address holds an IPv4 address.
    #[rhai_fn(global, pure, get = "is_ipv4")]
    pub fn is_ipv4(addr: &mut SocketAddr) -> bool {
        addr.is_ipv4()
    }

    /// Returns true if the socket address holds an IPv6 address.
    #[rhai_fn(global, pure, get = "is_ipv6")]
    pub fn is_ipv6(addr: &mut SocketAddr) -> bool {
        addr.is_ipv6()
    }

    /// Returns the port number associated with this socket address.
    #[rhai_fn(global, pure, return_raw)]
    pub fn port(addr: &mut SocketAddr) -> Result<rhai::INT, Box<EvalAltResult>> {
        convert_to_int(addr.port())
    }
}
