### ⚠️ **Master branch is unstable, use at your own risk**

# About `rhai-net`

[![License](https://img.shields.io/crates/l/rhai-net)](https://github.com/license/rhaiscript/rhai-net)
[![crates.io](https://img.shields.io/crates/v/rhai-net?logo=rust)](https://crates.io/crates/rhai-net/)
[![crates.io](https://img.shields.io/crates/d/rhai-net?logo=rust)](https://crates.io/crates/rhai-net/)
[![API Docs](https://docs.rs/rhai-net/badge.svg?logo=docs-rs)](https://docs.rs/rhai-net/)

This crate provides networking functionality for the [Rhai] scripting language.

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-net = "0.0.1"
```

### [Rhai] script

```js
let listener = tcp_listen("127.0.0.1:8080");
let stream = listener.accept(); // Blocks until new connection opened.
stream.write("Hello!");
let response = stream.read_string(0); // Read until connection closed.
print(response);
```

### Rust source

```rust,no_run
use rhai::{Engine, EvalAltResult};
use rhai::packages::Package;
use rhai_net::NetworkingPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create Rhai scripting engine
    let mut engine = Engine::new();

    // Create networking package and add the package into the engine
    let package = NetworkingPackage::new();
    package.register_into_engine(&mut engine);

    // Print the contents of the file `Cargo.toml`.
    let contents = engine.eval::<String>(r#"tcp_connect("127.0.0.1:8080").read_string()"#)?;
    println!("{}", contents);

    Ok(())
}
```

## Features

|  Feature   | Default  | Description                                          |
| :--------: | :------: | ---------------------------------------------------- |
| `no_index` | disabled | Enables support for `no_index` builds of [Rhai]      |
| `metadata` | disabled | Enables support for generating package documentation |

[Rhai]: https://rhai.rs