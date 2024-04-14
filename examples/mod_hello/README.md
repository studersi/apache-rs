mod_hello
=========

A simple Apache httpd handler similar to the minimal handler described in [Developing modules for the Apache HTTP Server 2.4 - top
Building a handler](https://httpd.apache.org/docs/2.4/developer/modguide.html#handling).



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


### Hurl
> https://hurl.dev

* Install Hurl for testing
```bash
cargo install --locked hurl
```



Build
-----

* Build
```bash
cargo build
```

* Install
```bash
cp ./target/debug/libmod_hello.so ./target/debug/mod_hello.so
sudo apxs -i -a -n hello ./target/debug/mod_hello.so
```



Tests
-----

* Start apache
```bash
apache2 -f "$(pwd)/tests/apache2.conf" -X
```

* Navigate to http://localhost:8080/hello

* Run Hurl tests
```bash
hurl --test tests/*.hurl
```
