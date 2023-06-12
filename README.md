apache-rs
=========

Rust FFI for creating Apache httpd modules.



Setup
-----

### bindgen
> https://rust-lang.github.io/rust-bindgen/requirements.html

* Install system packages
```bash
sudo apt install llvm-dev libclang-dev clang
```

### Apache httpd
* Install system packages
```bash
sudo apt install apache2-dev libapr1-dev
```



Build
-----

### FFI 
* Build project
```bash
cargo build
cargo test --lib
```

### Apache httpd module
* Build Apache httpd module `mod_example`
```bash
cd examples/mod_example/
cargo build
```



Test
----

### Locally
* Install module
```bash
cd examples/mod_example/
cargo build
sudo apxs -i -a -n example ./target/debug/libmod_example.so
```
* Add module configuration
```bash
sudo tee /etc/apache2/mods-enabled/example.conf > /dev/null <<EOT
<IfModule example_module>
    <Location "/example">
        SetHandler example-handler
    </Location>
</IfModule>
EOT
```
* Restart Apache httpd to load configuration
```bash
sudo systemctl restart apache2
```
* Navigate to http://localhost/example

### Container
