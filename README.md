apache-rs
=========

Rust FFI for creating Apache httpd modules.

There is a bundled example implementing `mod_hello` from the Apache httpd documentation: https://httpd.apache.org/docs/2.4/developer/modguide.html.



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

* Build Apache httpd module `mod_hello`

```bash
cd examples/mod_hello/
cargo build
```



Test
----


### Locally

* Build and install modules

```bash
cd examples/mod_hello/
cargo build
sudo apxs -i -a -n example ./target/debug/libmod_hello.so
```

```bash
cd examples/mod_sum/
cargo build
sudo apxs -i -a -n sum ./target/debug/libmod_sum.so
```

* Add module configurations

```bash
sudo tee /etc/apache2/mods-available/hello.conf > /dev/null <<EOT
<IfModule hello_module>
    <Location "/hello">
        SetHandler hello-handler
    </Location>
</IfModule>
EOT
sudo a2enmod hello
```

```bash
sudo tee /etc/apache2/mods-available/sum.conf > /dev/null <<EOT
<IfModule sum_module>
    AddHandler sum-handler .sum
</IfModule>
EOT
sudo a2enmod sum
```

* Restart Apache httpd to load configuration

```bash
sudo systemctl restart apache2
```

* Navigate to http://localhost/example

* Navigate to http://localhost/any/path.sum
