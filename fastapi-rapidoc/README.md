# fastapi-rapidoc

This crate works as a bridge between [fastapi](https://docs.rs/fastapi/latest/fastapi/) and [RapiDoc](https://rapidocweb.com/) OpenAPI visualizer.

[![Fastapi build](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml/badge.svg)](https://github.com/nxpkg/fastapi/actions/workflows/build.yaml)
[![crates.io](https://img.shields.io/crates/v/fastapi-rapidoc.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/fastapi-rapidoc)
![rustc](https://img.shields.io/static/v1?label=rustc&message=1.75&color=orange&logo=rust)

Fastapi-rapidoc provides simple mechanism to transform OpenAPI spec resource to a servable HTML
file which can be served via [predefined framework integration](#examples) or used
[standalone](#using-standalone) and served manually.

You may find fullsize examples from fastapi's Github [repository][examples].

# Crate Features

* **actix-web** Allows serving `RapiDoc` via _**`actix-web`**_. `version >= 4`
* **rocket** Allows serving `RapiDoc` via _**`rocket`**_. `version >=0.5`
* **axum** Allows serving `RapiDoc` via _**`axum`**_. `version >=0.7`

# Install

Use RapiDoc only without any boiler plate implementation.
```toml
[dependencies]
fastapi-rapidoc = "5"
```

Enable actix-web integration with RapiDoc.
```toml
[dependencies]
fastapi-rapidoc = { version = "5", features = ["actix-web"] }
```

# Using standalone

Fastapi-rapidoc can be used standalone as simply as creating a new `RapiDoc` instance and then
serving it by what ever means available as `text/html` from http handler in your favourite web
framework.

`RapiDoc::to_html` method can be used to convert the `RapiDoc` instance to a servable html
file.
```rust
let rapidoc = RapiDoc::new("/api-docs/openapi.json");

// Then somewhere in your application that handles http operation.
// Make sure you return correct content type `text/html`.
let rapidoc_handler = move || {
    rapidoc.to_html()
};
```

# Customization

Fastapi-rapidoc can be customized and configured only via `RapiDoc::custom_html` method. This
method empowers users to use a custom HTML template to modify the looks of the RapiDoc UI.

* [All allowed RapiDoc configuration options][rapidoc_api]
* [Default HTML template][rapidoc_quickstart]

The template should contain _**`$specUrl`**_ variable which will be replaced with user defined
OpenAPI spec url provided with `RapiDoc::new` function when creating a new `RapiDoc`
instance. Variable will be replaced during `RapiDoc::to_html` function execution.

_**Overriding the HTML template with a custom one.**_
```rust
let html = "...";
RapiDoc::new("/api-docs/openapi.json").custom_html(html);
```

# Examples

_**Serve `RapiDoc` via `actix-web` framework.**_
```rust
use actix_web::App;
use fastapi_rapidoc::RapiDoc;

App::new()
    .service(
        RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc")
    );
```

_**Serve `RapiDoc` via `rocket` framework.**_
```rust
use fastapi_rapidoc::RapiDoc;

rocket::build()
    .mount(
        "/",
        RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"),
    );
```

_**Serve `RapiDoc` via `axum` framework.**_
```rust
use axum::Router;
use fastapi_rapidoc::RapiDoc;

let app = Router::<S>::new()
    .merge(
        RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc")
    );
```

# License

Licensed under either of [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, shall be dual licensed, without any additional terms or conditions.

[rapidoc_api]: <https://rapidocweb.com/api.html>
[examples]: <https://github.com/nxpkg/fastapi/tree/master/examples>
[rapidoc_quickstart]: <https://rapidocweb.com/quickstart.html>
