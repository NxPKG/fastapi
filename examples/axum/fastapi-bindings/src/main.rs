use std::io;
use std::net::Ipv4Addr;

use fastapi::OpenApi;
use fastapi_axum::router::OpenApiRouter;
use fastapi_axum::routes;
use fastapi_swagger_ui::SwaggerUi;
use tokio::net::TcpListener;

const CUSTOMER_TAG: &str = "customer";
const ORDER_TAG: &str = "order";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CUSTOMER_TAG, description = "Customer API endpoints"),
        (name = ORDER_TAG, description = "Order API endpoints")
    )
)]
struct ApiDoc;

/// Get health of the API.
#[fastapi::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .nest("/api/customer", customer::router())
        .nest("/api/order", order::router())
        .routes(routes!(
            inner::secret_handlers::get_secret,
            inner::secret_handlers::post_secret
        ))
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
    axum::serve(listener, router).await
}

mod customer {
    use axum::Json;
    use fastapi::ToSchema;
    use fastapi_axum::router::OpenApiRouter;
    use fastapi_axum::routes;
    use serde::Serialize;

    /// This is the customer
    #[derive(ToSchema, Serialize)]
    struct Customer {
        name: String,
    }

    /// expose the Customer OpenAPI to parent module
    pub fn router() -> OpenApiRouter {
        OpenApiRouter::new().routes(routes!(get_customer))
    }

    /// Get customer
    ///
    /// Just return a static Customer object
    #[fastapi::path(get, path = "", responses((status = OK, body = Customer)), tag = super::CUSTOMER_TAG)]
    async fn get_customer() -> Json<Customer> {
        Json(Customer {
            name: String::from("Bill Book"),
        })
    }
}

mod order {
    use axum::Json;
    use fastapi::ToSchema;
    use fastapi_axum::router::OpenApiRouter;
    use fastapi_axum::routes;
    use serde::{Deserialize, Serialize};

    /// This is the order
    #[derive(ToSchema, Serialize)]
    struct Order {
        id: i32,
        name: String,
    }

    #[derive(ToSchema, Deserialize, Serialize)]
    struct OrderRequest {
        name: String,
    }

    /// expose the Order OpenAPI to parent module
    pub fn router() -> OpenApiRouter {
        OpenApiRouter::new().routes(routes!(get_order, create_order))
    }

    /// Get static order object
    #[fastapi::path(get, path = "", responses((status = OK, body = Order)), tag = super::ORDER_TAG)]
    async fn get_order() -> Json<Order> {
        Json(Order {
            id: 100,
            name: String::from("Bill Book"),
        })
    }

    /// Create an order.
    ///
    /// Create an order by basically passing through the name of the request with static id.
    #[fastapi::path(post, path = "", responses((status = OK, body = Order)), tag = super::ORDER_TAG)]
    async fn create_order(Json(order): Json<OrderRequest>) -> Json<Order> {
        Json(Order {
            id: 120,
            name: order.name,
        })
    }
}

mod inner {
    pub mod secret_handlers {

        /// This is some secret inner handler
        #[fastapi::path(get, path = "/api/inner/secret", responses((status = OK, body = str)))]
        pub async fn get_secret() -> &'static str {
            "secret"
        }

        /// Post some secret inner handler
        #[fastapi::path(post, path = "/api/inner/secret", responses((status = OK)))]
        pub async fn post_secret() {
            println!("You posted a secret")
        }
    }
}
