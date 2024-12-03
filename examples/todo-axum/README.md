# todo-axum ~ fastapi with fastapi-swagger-ui, fastapi-redoc and fastapi-rapidoc example

This is a demo `axum` application with in-memory storage to manage Todo items. The API
demonstrates `fastapi` with `fastapi-swagger-ui` functionalities.

For security restricted endpoints the super secret API key is: `fastapi-rocks`.

Just run command below to run the demo application and browse to `http://localhost:8080/swagger-ui/`.

If you prefer Redoc just head to `http://localhost:8080/redoc` and view the Open API.

RapiDoc can be found from `http://localhost:8080/rapidoc`.

Scalar can be reached on `http://localhost:8080/scalar`.

```bash
cargo run
```
