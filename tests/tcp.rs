use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    time::Duration,
};

use rhai::{packages::Package, Engine, EvalAltResult, Scope};
use rhai_net::{NetworkingPackage, SharedTcpStream};

#[test]
fn test_tcp() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Create listener that rhai will connect / write to.
    let _listen = std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        // Read test
        let (mut first_test, _) = listener.accept().unwrap();
        first_test
            .set_write_timeout(Some(Duration::new(1, 0)))
            .unwrap();
        first_test.write(b"from rust").unwrap();
        first_test.shutdown(Shutdown::Both).unwrap();

        // Write test
        let (mut second_test, _) = listener.accept().unwrap();
        let mut buf = String::new();
        second_test.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "from rhai");
        second_test.shutdown(Shutdown::Both).unwrap();
    });

    // Register our networking package.
    let package = NetworkingPackage::new();
    package.register_into_engine(&mut engine);

    // Create a conn and read string.
    assert_eq!(
        engine.eval::<String>(r#"tcp_connect(addr("127.0.0.1:8080")).read_string(9)"#)?,
        "from rust"
    );

    // Create a conn and write string.
    assert_eq!(
        engine.eval::<rhai::INT>(
            r#"let stream = tcp_connect(addr("127.0.0.1:8080"));
        let res = stream.write("from rhai");
        res"#
        )?,
        9
    );

    // Make sure we catch asserts from listener thread.
    _listen.join().unwrap();

    Ok(())
}

#[test]
fn test_tcp_server() -> Result<(), Box<EvalAltResult>> {
    // Create client that will connect / write to rhai.
    let _server = std::thread::spawn(|| {
        let mut engine = Engine::new();

        // Register our networking package.
        let package = NetworkingPackage::new();
        package.register_into_engine(&mut engine);

        let mut scope = Scope::new();
        scope.push(
            "stream",
            engine
                .eval::<SharedTcpStream>(r#"tcp_listen(addr("127.0.0.1:8081")).accept()"#)
                .unwrap(),
        );

        // Read test.
        assert_eq!(
            engine
                .eval_with_scope::<String>(&mut scope, r#"stream.read_string(9)"#)
                .unwrap(),
            "from rust"
        );

        // Write test.
        assert_eq!(
            engine
                .eval_with_scope::<rhai::INT>(&mut scope, r#"stream.write("from rhai")"#)
                .unwrap(),
            9
        );
    });

    let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();

    // Read test
    stream.set_write_timeout(Some(Duration::new(1, 0))).unwrap();
    stream.write(b"from rust").unwrap();

    // Write test
    let mut buf = String::new();
    stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    stream.read_to_string(&mut buf).unwrap();
    assert_eq!(buf, "from rhai");

    // Make sure we catch asserts from rhai server thread.
    _server.join().unwrap();

    Ok(())
}
