#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Fastapi axum brings `fastapi` and `axum` closer together by the way of providing an ergonomic API that is extending on
//! the `axum` API. It gives a natural way to register handlers known to `axum` and also simultaneously generates OpenAPI
//! specification from the handlers.
//!
//! ## Crate features
//!
//! - **`debug`**: Implement debug traits for types.
//!
//! ## Install
//!
//! Add dependency declaration to `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! fastapi-axum = "0.1.1"
//! ```
//!
//! ## Examples
//!
//! _**Use [`OpenApiRouter`][router] to collect handlers with _`#[fastapi::path]`_ macro to compose service and form OpenAPI spec.**_
//!
//! ```rust
//! # use axum::Json;
//! # use fastapi::openapi::OpenApi;
//! # use fastapi_axum::{routes, PathItemExt, router::OpenApiRouter};
//!  #[derive(fastapi::ToSchema, serde::Serialize)]
//!  struct User {
//!      id: i32,
//!  }
//!
//!  #[fastapi::path(get, path = "/user", responses((status = OK, body = User)))]
//!  async fn get_user() -> Json<User> {
//!     Json(User { id: 1 })
//!  }
//!  
//!  let (router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
//!      .routes(routes!(get_user))
//!      .split_for_parts();
//! ```
//!
//! [router]: router/struct.OpenApiRouter.html

pub mod router;

use axum::routing::MethodFilter;
use fastapi::openapi::HttpMethod;

/// Extends [`fastapi::openapi::path::PathItem`] by providing conversion methods to convert this
/// path item type to a [`axum::routing::MethodFilter`].
pub trait PathItemExt {
    /// Convert this path item type to a [`axum::routing::MethodFilter`].
    ///
    /// Method filter is used with handler registration on [`axum::routing::MethodRouter`].
    fn to_method_filter(&self) -> MethodFilter;
}

impl PathItemExt for HttpMethod {
    fn to_method_filter(&self) -> MethodFilter {
        match self {
            HttpMethod::Get => MethodFilter::GET,
            HttpMethod::Put => MethodFilter::PUT,
            HttpMethod::Post => MethodFilter::POST,
            HttpMethod::Head => MethodFilter::HEAD,
            HttpMethod::Patch => MethodFilter::PATCH,
            HttpMethod::Trace => MethodFilter::TRACE,
            HttpMethod::Delete => MethodFilter::DELETE,
            HttpMethod::Options => MethodFilter::OPTIONS,
        }
    }
}

/// re-export paste so users do not need to add the dependency.
#[doc(hidden)]
pub use paste::paste;

/// Collect axum handlers annotated with [`fastapi::path`] to [`router::FastapiMethodRouter`].
///
/// [`routes`] macro will return [`router::FastapiMethodRouter`] which contains an
/// [`axum::routing::MethodRouter`] and currently registered paths. The output of this macro is
/// meant to be used together with [`router::OpenApiRouter`] which combines the paths and axum
/// routers to a single entity.
///
/// Only handlers collected with [`routes`] macro will get registered to the OpenApi.
///
/// # Panics
///
/// Routes registered via [`routes`] macro or via `axum::routing::*` operations are bound to same
/// rules where only one one HTTP method can can be registered once per call. This means that the
/// following will produce runtime panic from axum code.
///
/// ```rust,no_run
/// # use fastapi_axum::{routes, router::FastapiMethodRouter};
/// # use fastapi::path;
///  #[fastapi::path(get, path = "/search")]
///  async fn search_user() {}
///
///  #[fastapi::path(get, path = "")]
///  async fn get_user() {}
///
///  let _: FastapiMethodRouter = routes!(get_user, search_user);
/// ```
/// Since the _`axum`_ does not support method filter for `CONNECT` requests, using this macro with
/// handler having request method type `CONNECT` `#[fastapi::path(connect, path = "")]` will panic at
/// runtime.
///
/// # Examples
///
/// _**Create new `OpenApiRouter` with `get_user` and `post_user` paths.**_
/// ```rust
/// # use fastapi_axum::{routes, router::{OpenApiRouter, FastapiMethodRouter}};
/// # use fastapi::path;
///  #[fastapi::path(get, path = "")]
///  async fn get_user() {}
///
///  #[fastapi::path(post, path = "")]
///  async fn post_user() {}
///
///  let _: OpenApiRouter = OpenApiRouter::new().routes(routes!(get_user, post_user));
/// ```
#[macro_export]
macro_rules! routes {
    ( $handler:path $(, $tail:path)* ) => {
        {
            use $crate::PathItemExt;
            let mut paths = fastapi::openapi::path::Paths::new();
            let mut schemas = Vec::<(String, fastapi::openapi::RefOr<fastapi::openapi::schema::Schema>)>::new();
            let (path, item, types) = $crate::routes!(@resolve_types $handler : schemas);
            #[allow(unused_mut)]
            let mut method_router = types.iter().by_ref().fold(axum::routing::MethodRouter::new(), |router, path_type| {
                router.on(path_type.to_method_filter(), $handler)
            });
            paths.add_path_operation(&path, types, item);
            $( method_router = $crate::routes!( schemas: method_router: paths: $tail ); )*
            (schemas, paths, method_router)
        }
    };
    ( $schemas:tt: $router:ident: $paths:ident: $handler:path $(, $tail:tt)* ) => {
        {
            let (path, item, types) = $crate::routes!(@resolve_types $handler : $schemas);
            let router = types.iter().by_ref().fold($router, |router, path_type| {
                router.on(path_type.to_method_filter(), $handler)
            });
            $paths.add_path_operation(&path, types, item);
            router
        }
    };
    ( @resolve_types $handler:path : $schemas:tt ) => {
        {
            $crate::paste! {
                let path = $crate::routes!( @path [path()] of $handler );
                let mut operation = $crate::routes!( @path [operation()] of $handler );
                let types = $crate::routes!( @path [methods()] of $handler );
                let tags = $crate::routes!( @path [tags()] of $handler );
                $crate::routes!( @path [schemas(&mut $schemas)] of $handler );
                if !tags.is_empty() {
                    let operation_tags = operation.tags.get_or_insert(Vec::new());
                    operation_tags.extend(tags.iter().map(ToString::to_string));
                }
                (path, operation, types)
            }
        }
    };
    ( @path $op:tt of $part:ident $( :: $tt:tt )* ) => {
        $crate::routes!( $op : [ $part $( $tt )*] )
    };
    ( $op:tt : [ $first:tt $( $rest:tt )* ] $( $rev:tt )* ) => {
        $crate::routes!( $op : [ $( $rest )* ] $first $( $rev)* )
    };
    ( $op:tt : [] $first:tt $( $rest:tt )* ) => {
        $crate::routes!( @inverse $op : $first $( $rest )* )
    };
    ( @inverse $op:tt : $tt:tt $( $rest:tt )* ) => {
        $crate::routes!( @rev $op : $tt [$($rest)*] )
    };
    ( @rev $op:tt : $tt:tt [ $first:tt $( $rest:tt)* ] $( $reversed:tt )* ) => {
        $crate::routes!( @rev $op : $tt [ $( $rest )* ] $first $( $reversed )* )
    };
    ( @rev [$op:ident $( $args:tt )* ] : $handler:tt [] $($tt:tt)* ) => {
        {
            #[allow(unused_imports)]
            use fastapi::{Path, __dev::{Tags, SchemaReferences}};
            $crate::paste! {
                $( $tt :: )* [<__path_ $handler>]::$op $( $args )*
            }
        }
    };
    ( ) => {};
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use axum::extract::State;
    use fastapi::openapi::{Content, Ref, ResponseBuilder};
    use fastapi::PartialSchema;
    use router::*;

    #[fastapi::path(get, path = "/")]
    async fn root() {}

    // --- user

    #[fastapi::path(get, path = "/")]
    async fn get_user() {}

    #[fastapi::path(post, path = "/")]
    async fn post_user() {}

    #[fastapi::path(delete, path = "/")]
    async fn delete_user() {}

    #[fastapi::path(get, path = "/search")]
    async fn search_user() {}

    // --- customer

    #[fastapi::path(get, path = "/")]
    async fn get_customer() {}

    #[fastapi::path(post, path = "/")]
    async fn post_customer() {}

    #[fastapi::path(delete, path = "/")]
    async fn delete_customer() {}

    // test that with state handler compiles
    #[fastapi::path(get, path = "/search")]
    async fn search_customer(State(_s): State<String>) {}

    #[test]
    fn axum_router_nest_openapi_routes_compile() {
        let user_router: OpenApiRouter = OpenApiRouter::new()
            .routes(routes!(search_user))
            .routes(routes!(get_user, post_user, delete_user));

        let customer_router: OpenApiRouter = OpenApiRouter::new()
            .routes(routes!(get_customer, post_customer, delete_customer))
            .routes(routes!(search_customer))
            .with_state(String::new());

        let router = OpenApiRouter::new()
            .nest("/api/user", user_router)
            .nest("/api/customer", customer_router)
            .route("/", axum::routing::get(root));

        let _ = router.get_openapi();
    }

    #[test]
    fn openapi_router_with_openapi() {
        use fastapi::OpenApi;

        #[derive(fastapi::ToSchema)]
        #[allow(unused)]
        struct Todo {
            id: i32,
        }
        #[derive(fastapi::OpenApi)]
        #[openapi(components(schemas(Todo)))]
        struct Api;

        let mut router: OpenApiRouter = OpenApiRouter::with_openapi(Api::openapi())
            .routes(routes!(search_user))
            .routes(routes!(get_user));

        let paths = router.to_openapi().paths;
        let expected_paths = fastapi::openapi::path::PathsBuilder::new()
            .path(
                "/",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Get,
                    fastapi::openapi::path::OperationBuilder::new().operation_id(Some("get_user")),
                ),
            )
            .path(
                "/search",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Get,
                    fastapi::openapi::path::OperationBuilder::new()
                        .operation_id(Some("search_user")),
                ),
            );
        assert_eq!(expected_paths.build(), paths);
    }

    #[test]
    fn openapi_router_nest_openapi() {
        use fastapi::OpenApi;

        #[derive(fastapi::ToSchema)]
        #[allow(unused)]
        struct Todo {
            id: i32,
        }
        #[derive(fastapi::OpenApi)]
        #[openapi(components(schemas(Todo)))]
        struct Api;

        let router: router::OpenApiRouter =
            router::OpenApiRouter::with_openapi(Api::openapi()).routes(routes!(search_user));

        let customer_router: router::OpenApiRouter = router::OpenApiRouter::new()
            .routes(routes!(get_customer))
            .with_state(String::new());

        let mut router = router.nest("/api/customer", customer_router);
        let paths = router.to_openapi().paths;
        let expected_paths = fastapi::openapi::path::PathsBuilder::new()
            .path(
                "/api/customer",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Get,
                    fastapi::openapi::path::OperationBuilder::new()
                        .operation_id(Some("get_customer")),
                ),
            )
            .path(
                "/search",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Get,
                    fastapi::openapi::path::OperationBuilder::new()
                        .operation_id(Some("search_user")),
                ),
            );
        assert_eq!(expected_paths.build(), paths);
    }

    #[test]
    fn openapi_with_auto_collected_schemas() {
        #[derive(fastapi::ToSchema)]
        #[allow(unused)]
        struct Todo {
            id: i32,
        }

        #[fastapi::path(get, path = "/todo", responses((status = 200, body = Todo)))]
        async fn get_todo() {}

        let mut router: router::OpenApiRouter =
            router::OpenApiRouter::new().routes(routes!(get_todo));

        let openapi = router.to_openapi();
        let paths = openapi.paths;
        let schemas = openapi
            .components
            .expect("Router must have auto collected schemas")
            .schemas;

        let expected_paths = fastapi::openapi::path::PathsBuilder::new().path(
            "/todo",
            fastapi::openapi::PathItem::new(
                fastapi::openapi::path::HttpMethod::Get,
                fastapi::openapi::path::OperationBuilder::new()
                    .operation_id(Some("get_todo"))
                    .response(
                        "200",
                        ResponseBuilder::new().content(
                            "application/json",
                            Content::builder()
                                .schema(Some(Ref::from_schema_name("Todo")))
                                .build(),
                        ),
                    ),
            ),
        );
        let expected_schemas =
            BTreeMap::from_iter(std::iter::once(("Todo".to_string(), Todo::schema())));
        assert_eq!(expected_paths.build(), paths);
        assert_eq!(expected_schemas, schemas);
    }

    mod pets {

        #[fastapi::path(get, path = "/")]
        pub async fn get_pet() {}

        #[fastapi::path(post, path = "/")]
        pub async fn post_pet() {}

        #[fastapi::path(delete, path = "/")]
        pub async fn delete_pet() {}
    }

    #[test]
    fn openapi_routes_from_another_path() {
        let mut router: OpenApiRouter =
            OpenApiRouter::new().routes(routes!(pets::get_pet, pets::post_pet, pets::delete_pet));
        let paths = router.to_openapi().paths;

        let expected_paths = fastapi::openapi::path::PathsBuilder::new()
            .path(
                "/",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Get,
                    fastapi::openapi::path::OperationBuilder::new().operation_id(Some("get_pet")),
                ),
            )
            .path(
                "/",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Post,
                    fastapi::openapi::path::OperationBuilder::new().operation_id(Some("post_pet")),
                ),
            )
            .path(
                "/",
                fastapi::openapi::PathItem::new(
                    fastapi::openapi::path::HttpMethod::Delete,
                    fastapi::openapi::path::OperationBuilder::new()
                        .operation_id(Some("delete_pet")),
                ),
            );
        assert_eq!(expected_paths.build(), paths);
    }
}
