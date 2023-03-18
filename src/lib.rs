#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
//!
//! ## API
//!
//! The following functions are defined in this package:
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-net-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod addr;
pub(crate) mod tcp;
pub(crate) mod util;

// Re-export types.
pub use tcp::tcp_functions::{listener_functions::SharedTcpListener, SharedTcpStream};

def_package! {
    /// Package for networking operations.
    pub NetworkingPackage(lib) {
        combine_with_exported_module!(lib, "rhai_net_addr", addr::addr_functions);
        combine_with_exported_module!(lib, "rhai_net_tcp", tcp::tcp_functions);
    }
}
