# todo-warp ~ fastapi with fastapi-swagger-ui example

This is a demo `warp` application with in-memory storage to manage Todo items.
The API demonstrates `fastapi` with `fastapi-swagger-ui` functionalities.

This example is more bare minimum compared to `todo-actix`, since similarly same macro syntax is
supported, no matter the framework.

Purpose of this `warp` demo is to mainly demonstrate how `fastapi` and `fastapi-swagger-ui` can be integrated
with other frameworks as well.

For security restricted endpoints the super secret API key is: `fastapi-rocks`.

Just run command below to run the demo application and browse to `http://localhost:8080/swagger-ui/`.

```bash
cargo run
```

If you want to see some logging, you may prepend the command with `RUST_LOG=debug` as shown below.

```bash
RUST_LOG=debug cargo run
```