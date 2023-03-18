//! A simple example that spawns a client and server and says hello!
//! Requires that the feature "no-index" is not enabled.
use std::thread;

use rhai::{packages::Package, Engine};
use rhai_net::NetworkingPackage;

fn main() -> Result<(), ()> {
    let server_thread = thread::spawn(|| {
        let mut engine = Engine::new();

        // Register our networking package.
        let package = NetworkingPackage::new();
        package.register_into_engine_as(&mut engine, "net");

        engine
            .run(
                r#"
        let listener = net::tcp_listen("127.0.0.1:8080");
        let stream = listener.accept();
        let b = stream.read_string(4);
        print(b)"#,
            )
            .unwrap();
    });

    let client_thread = thread::spawn(|| {
        let mut engine = Engine::new();

        // Register our networking package.
        let package = NetworkingPackage::new();
        package.register_into_engine_as(&mut engine, "net");

        engine
            .run(
                r#"
        let stream = net::tcp_connect("127.0.0.1:8080");
        stream.write("test");
        stream.write("this wont print!");
        "#,
            )
            .unwrap();
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();

    Ok(())
}
