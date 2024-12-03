# fastapi examples

This is folder contain a set of examples of fastapi library which should help people to get started
with the library.

All examples have their own `README.md`, and can be seen using two steps:

1. Run `cargo run`
2. Browse to `http://localhost:8080/swagger-ui/` or `http://localhost:8080/redoc` or `http://localhost:8080/rapidoc`.

`todo-actix`, `todo-axum` and `rocket-todo` have Swagger UI, Redoc, RapiDoc, and Scalar setup, others have Swagger UI 
if not explicitly stated otherwise.

Even if there is no example for your favourite framework, `fastapi` can be used with any
web framework which supports decorating functions with macros similarly to the **warp** and **tide** examples.

## Community examples

- **[graphul](https://github.com/graphul-rs/graphul/tree/main/examples/fastapi-swagger-ui)**
- **[salvo](https://github.com/salvo-rs/salvo/tree/main/examples/todos-fastapi)**
- **[viz](https://github.com/viz-rs/viz/tree/main/examples/routing/openapi)**
- **[ntex](https://github.com/leon3s/ntex-rest-api-example)**

## Examples Directory Structure

```
examples/
├── README.md                 # High-level overview of all examples
├── actix-web/                # Grouping examples by framework/library
│   ├── README.md             # Description of Actix-specific examples
│   ├── multiple-api-docs/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── README.md
│   └── scopes-binding/
│       ├── Cargo.toml
│       ├── src/
│       └── README.md
├── axum/
│   ├── README.md
│   ├── fastapi-bindings/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── README.md
│   └── nesting-vendored/
│       ├── Cargo.toml
│       ├── src/
│       ├── Dockerfile
│       └── README.md
├── generic/
│   ├── README.md             # For framework-agnostic examples
│   ├── raw-json/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── README.md
│   └── todo/
│       ├── actix/
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   └── README.md
│       ├── axum/
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   └── README.md
│       ├── tide/
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   └── README.md
│       ├── warp/
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   └── README.md
│       └── warp-rapidoc/
│           ├── Cargo.toml
│           ├── src/
│           └── README.md
├── rocket/
│   ├── README.md
│   ├── todo/
│       ├── Cargo.toml
│       ├── src/
│       └── README.md
└── warp/
    ├── README.md
    ├── multiple-api-docs/
    │   ├── Cargo.toml
    │   ├── src/
    │   └── README.md
    └── redoc-with-file-config/
        ├── Cargo.toml
        ├── src/
        ├── redoc.json
        └── README.md
```
