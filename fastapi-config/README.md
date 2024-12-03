# fastapi-config

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-config.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-config)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

This crate provides global configuration capabilities for `fastapi`.

## Config options

* Define rust type aliases for `fastapi` with `.alias_for(...)` method.
* Define schema collect mode for `fastapi` with `.schema_collect(...)` method.
  * `SchemaCollect:All` will collect all schemas from usages including inlined with `inline(T)`
  * `SchemaCollect::NonInlined` will only collect non inlined schemas from usages.

> [!WARNING]
> The build config will be stored to projects `OUTPUT` directory. It is then read from there via `OUTPUT` environment
> variable which will return **any instance** rust compiler might find at that time (Whatever the `OUTPUT` environment variable points to).
> **Be aware** that sometimes you might face a situation where the config is not aligned with your Rust aliases. 
> This might need you to change something on your code before changed config might apply.

## Install

Add dependency declaration to `Cargo.toml`.

```toml
[build-dependencies]
fastapi-config = "0.1"
```

## Examples

Create `build.rs` file with following content, then in your code you can just use `MyType` as 
alternative for `i32`.

```rust
use fastapi_config::Config;

fn main() {
    Config::new()
        .alias_for("MyType", "i32")
        .write_to_file();
}
```

See full [example for fastapi-config](../examples/fastapi-config-test/).

## License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.
