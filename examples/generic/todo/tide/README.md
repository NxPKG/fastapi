# todo-tide ~ fastapi with fastapi-swagger-ui example

This is a demo `tide` application with in-memory storage to manage Todo items. The API
demonstrates `fastapi` with `fastapi-swagger-ui` functionalities.

For security restricted endpoints the super secret API key is: `fastapi-rocks`.

Just run command below to run the demo application and browse to `http://localhost:8080/swagger-ui/index.html`.

```bash
cargo run
```

If you want to see some logging, you may prepend the command with `RUST_LOG=debug` as shown below.

```bash
RUST_LOG=debug cargo run
```
