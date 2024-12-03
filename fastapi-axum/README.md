# fastapi-axum - Bindings for Axum and fastapi

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-axum.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-axum)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

Fastapi axum brings `fastapi` and `axum` closer together by the way of providing an ergonomic API that is extending on
the `axum` API. It gives a natural way to register handlers known to `axum` and also simultaneously generates OpenAPI
specification from the handlers.

## Crate features

- **`debug`**: Implement debug traits for types.

## Install

Add dependency declaration to `Cargo.toml`.

```toml
[dependencies]
fastapi-axum = "0.1"
```

## Examples

Use `OpenApiRouter` to collect handlers with `#[fastapi::path]` macro to compose service and form OpenAPI spec.

```rust
use fastapi_axum::{routes, PathItemExt, router::OpenApiRouter};

#[derive(fastapi::ToSchema)]
struct User {
    id: i32,
}

#[fastapi::path(get, path = "/user", responses((status = OK, body = User)))]
async fn get_user() -> Json<User> {
    Json(User { id: 1 })
}

let (router, api) = OpenApiRouter::new()
    .routes(routes!(get_user))
    .split_for_parts();
```

## License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.
