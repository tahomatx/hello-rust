# hello-rust

```sh
cargo install rustfmt
```

```sh
docker run -it --rm rust -v ${PWD}:/work -w /work rust cargo run
docker run --rm --user "$(id -u)":"$(id -g)" -v ${PWD}:/rust -w /rust rust cargo build
docker run --rm -v ${PWD}:/rust -w /rust rust cargo build
```
