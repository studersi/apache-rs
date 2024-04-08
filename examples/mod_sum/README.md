mod_sum
=======

A simple Apache httpd handler similar to the hashing handler described in [Developing modules for the Apache HTTP Server 2.4 - top
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
cp ./target/debug/libmod_sum.so ./target/debug/mod_sum.so
sudo apxs -i -a -n sum ./target/debug/mod_sum.so
```



Tests
-----

* Start apache
```bash
apache2 -f "$(pwd)/tests/apache2.conf" -X
```

* Navigate to http://localhost:8080/index.html.sum
* Navigate to http://localhost:8080/index.html.sum?digest=invalid
* Navigate to http://localhost:8080/index.html.sum?digest=md5
* Navigate to http://localhost:8080/index.html.sum?digest=sha1

* Run Hurl tests
```bash
hurl --test tests/*.hurl
```
