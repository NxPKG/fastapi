# fastapi-scalar

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-scalar.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-scalar)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

This crate works as a bridge between [fastapi](https://docs.rs/fastapi/latest/fastapi/) and [Scalar](https://scalar.com/) OpenAPI visualizer.

Fastapi-scalar provides simple mechanism to transform OpenAPI spec resource to a servable HTML
file which can be served via [predefined framework integration](#examples) or used
[standalone](#using-standalone) and served manually.

You may find fullsize examples from fastapi's Github [repository][examples].

# Crate Features

* **actix-web** Allows serving `Scalar` via _**`actix-web`**_. `version >= 4`
* **rocket** Allows serving `Scalar` via _**`rocket`**_. `version >=0.5`
* **axum** Allows serving `Scalar` via _**`axum`**_. `version >=0.7`

# Install

Use Scalar only without any boiler plate implementation.
```toml
[dependencies]
fastapi-scalar = "0.2"
```

Enable actix-web integration with Scalar.
```toml
[dependencies]
fastapi-scalar = { version = "0.2", features = ["actix-web"] }
```

# Using standalone

Fastapi-scalar can be used standalone as simply as creating a new `Scalar` instance and then
serving it by what ever means available as `text/html` from http handler in your favourite web
framework.

`Scalar::to_html` method can be used to convert the `Scalar` instance to a servable html
file.
```rust
let scalar = Scalar::new(ApiDoc::openapi());

// Then somewhere in your application that handles http operation.
// Make sure you return correct content type `text/html`.
let scalar = move || async {
    scalar.to_html()
};
```

# Customization

Scalar supports customization via [`Scalar::custom_html`] method which allows overriding the
default HTML template with customized one. 

**See more about configuration options.**

* [Quick HTML configuration instructions](https://github.com/scalar/scalar/blob/main/documentation/integrations/html.md)
* [Configuration options](https://github.com/scalar/scalar/blob/main/documentation/configuration.md)
* [Themes](https://github.com/scalar/scalar/blob/main/documentation/themes.md)

The HTML template must contain **`$spec`** variable which will be overridden during
`Scalar::to_html` execution.

* **`$spec`** Will be the `Spec` that will be rendered via `Scalar`.

_**Overriding the HTML template with a custom one.**_
```rust
# use fastapi_redoc::Redoc;
# use fastapi::OpenApi;
# use serde_json::json;
# #[derive(OpenApi)]
# #[openapi()]
# struct ApiDoc;
#
let html = "...";
Redoc::new(ApiDoc::openapi()).custom_html(html);
```

# Examples

_**Serve `Scalar` via `actix-web` framework.**_
```rust
use actix_web::App;
use fastapi_scalar::{Scalar, Servable};

App::new().service(Scalar::with_url("/scalar", ApiDoc::openapi()));
```

_**Serve `Scalar` via `rocket` framework.**_
```rust
use fastapi_scalar::{Scalar, Servable};

rocket::build()
    .mount(
        "/",
        Scalar::with_url("/scalar", ApiDoc::openapi()),
    );
```

_**Serve `Scalar` via `axum` framework.**_
 ```rust
 use axum::Router;
 use fastapi_scalar::{Scalar, Servable};

 let app = Router::<S>::new()
     .merge(Scalar::with_url("/scalar", ApiDoc::openapi()));
```

_**Use `Scalar` to serve OpenAPI spec from url.**_
```rust
Scalar::new(
  "https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml")
```

_**Use `Scalar` to serve custom OpenAPI spec using serde's `json!()` macro.**_
```rust
Scalar::new(json!({"openapi": "3.1.0"}));
```

# License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.

[examples]: <https://github.com/nxpkg/fastapi/tree/master/examples>
