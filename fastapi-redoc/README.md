# fastapi-redoc

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-redoc.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-redoc)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

This crate works as a bridge between [fastapi](https://docs.rs/fastapi/latest/fastapi/) and [Redoc](https://redocly.com/) OpenAPI visualizer.

Fastapi-redoc provides simple mechanism to transform OpenAPI spec resource to a servable HTML
file which can be served via [predefined framework integration](#examples) or used
[standalone](#using-standalone) and served manually.

You may find fullsize examples from fastapi's Github [repository][examples].

# Crate Features

* **actix-web** Allows serving `Redoc` via _**`actix-web`**_. `version >= 4`
* **rocket** Allows serving `Redoc` via _**`rocket`**_. `version >=0.5`
* **axum** Allows serving `Redoc` via _**`axum`**_. `version >=0.7`

# Install

Use Redoc only without any boiler plate implementation.
```toml
[dependencies]
fastapi-redoc = "5"
```

Enable actix-web integration with Redoc.
```toml
[dependencies]
fastapi-redoc = { version = "5", features = ["actix-web"] }
```

# Using standalone

Fastapi-redoc can be used standalone as simply as creating a new `Redoc` instance and then
serving it by what ever means available as `text/html` from http handler in your favourite web
framework.

`Redoc::to_html` method can be used to convert the `Redoc` instance to a servable html
file.
```rust
let redoc = Redoc::new(ApiDoc::openapi());

// Then somewhere in your application that handles http operation.
// Make sure you return correct content type `text/html`.
let redoc_handler = move || async {
    redoc.to_html()
};
```

# Customization

Fastapi-redoc enables full customization support for [Redoc][redoc] according to what can be
customized by modifying the HTML template and [configuration options](#configuration).

The default [HTML template][redoc_html_quickstart] can be fully overridden to ones liking with
`Redoc::custom_html` method. The HTML template **must** contain **`$spec`** and **`$config`**
variables which are replaced during `Redoc::to_html` execution.

* **`$spec`** Will be the `Spec` that will be rendered via [Redoc][redoc].
* **`$config`** Will be the current `Config`. By default this is `EmptyConfig`.

_**Overriding the HTML template with a custom one.**_
```rust
let html = "...";
Redoc::new(ApiDoc::openapi()).custom_html(html);
```

# Configuration

Redoc can be configured with JSON either inlined with the `Redoc` declaration or loaded from
user defined file with `FileConfig`.

* [All supported Redoc configuration options][redoc_config].

_**Inlining the configuration.**_
```rust
Redoc::with_config(ApiDoc::openapi(), || json!({ "disableSearch": true }));
```

_**Using `FileConfig`.**_
```rust
Redoc::with_config(ApiDoc::openapi(), FileConfig);
```

Read more details in `Config`.

# Examples

_**Serve `Redoc` via `actix-web` framework.**_
```rust
use actix_web::App;
use fastapi_redoc::{Redoc, Servable};

App::new().service(Redoc::with_url("/redoc", ApiDoc::openapi()));
```

_**Serve `Redoc` via `rocket` framework.**_
```rust
use fastapi_redoc::{Redoc, Servable};

rocket::build()
    .mount(
        "/",
        Redoc::with_url("/redoc", ApiDoc::openapi()),
    );
```

_**Serve `Redoc` via `axum` framework.**_
 ```rust
 use axum::Router;
 use fastapi_redoc::{Redoc, Servable};

 let app = Router::<S>::new()
     .merge(Redoc::with_url("/redoc", ApiDoc::openapi()));
```

_**Use `Redoc` to serve OpenAPI spec from url.**_
```rust
Redoc::new(
  "https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml")
```

_**Use `Redoc` to serve custom OpenAPI spec using serde's `json!()` macro.**_
```rust
Redoc::new(json!({"openapi": "3.1.0"}));
```

# License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.

[redoc]: <https://redocly.com/>
[redoc_html_quickstart]: <https://redocly.com/docs/redoc/quickstart/>
[redoc_config]: <https://redocly.com/docs/api-reference-docs/configuration/functionality/#configuration-options-for-api-docs>
[examples]: <https://github.com/nxpkg/fastapi/tree/master/examples>
