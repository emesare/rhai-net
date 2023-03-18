use std::net::SocketAddr;

use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_net::NetworkingPackage;

#[test]
fn test_addr() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our networking package.
    let package = NetworkingPackage::new();
    package.register_into_engine(&mut engine);

    // Create a ipv4 socket address.
    assert_eq!(
        engine.eval::<SocketAddr>(r#"addr("127.0.0.1:8080")"#)?,
        SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)
    );

    // Create a ipv6 socket address using `addr_with_port` override.
    assert_eq!(
        engine.eval::<SocketAddr>(r#"addr("::1", 8080)"#)?,
        SocketAddr::new("::1".parse().unwrap(), 8080)
    );

    Ok(())
}
