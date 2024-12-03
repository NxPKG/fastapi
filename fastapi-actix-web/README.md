# fastapi-actix-web - Bindings for Actix Web and fastapi

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-actix-web.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-actix-web)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

This crate implements necessary bindings for automatically collecting `paths` and `schemas` recursively from Actix Web
`App`, `Scope` and `ServiceConfig`. It provides natural API reducing duplication and support for scopes while generating
OpenAPI specification without the need to declare `paths` and `schemas` to `#[openapi(...)]` attribute of `OpenApi` derive.

Currently only `service(...)` calls supports automatic collection of schemas and paths. Manual routes via `route(...)` or
`Route::new().to(...)` is not supported.

## Install

Add dependency declaration to `Cargo.toml`.

```toml
[dependencies]
fastapi-actix-web = "0.1"
```

## Examples

Collect handlers annotated with `#[fastapi::path]` recursively from `service(...)` calls to compose OpenAPI spec.

```rust
use actix_web::{get, App};
use fastapi_actix_web::{scope, AppExt};

#[derive(fastapi::ToSchema)]
struct User {
    id: i32,
}

#[fastapi::path(responses((status = OK, body = User)))]
#[get("/user")]
async fn get_user() -> Json<User> {
    Json(User { id: 1 })
}

let (_, mut api) = App::new()
    .into_fastapi_app()
    .service(scope::scope("/api/v1").service(get_user))
    .split_for_parts();
```

## License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.
