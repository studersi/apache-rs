apache-rs
=========

Rust FFI for creating Apache httpd modules.



Build
-----
* Install system packages
```bash
sudo apt install apache2-dev libapr1-dev
```
* Build project
```bash
cargo build
cargo test --lib
```
* Build Apache httpd module `mod_example`
```bash
cd examples/mod_example/
cargo build
```
