//! This crate implements necessary bindings for automatically collecting `paths` and `schemas` recursively from Actix Web
//! `App`, `Scope` and `ServiceConfig`. It provides natural API reducing duplication and support for scopes while generating
//! OpenAPI specification without the need to declare `paths` and `schemas` to `#[openapi(...)]` attribute of `OpenApi` derive.
//!
//! Currently only `service(...)` calls supports automatic collection of schemas and paths. Manual routes via `route(...)` or
//! `Route::new().to(...)` is not supported.
//!
//! ## Install
//!
//! Add dependency declaration to `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! fastapi-actix-web = "0.1.1"
//! ```
//!
//! ## Examples
//!
//! _**Collect handlers annotated with `#[fastapi::path]` recursively from `service(...)` calls to compose OpenAPI spec.**_
//!
//! ```rust
//! use actix_web::web::Json;
//! use actix_web::{get, App};
//! use fastapi_actix_web::{scope, AppExt};
//!
//! #[derive(fastapi::ToSchema, serde::Serialize)]
//! struct User {
//!     id: i32,
//! }
//!
//! #[fastapi::path(responses((status = OK, body = User)))]
//! #[get("/user")]
//! async fn get_user() -> Json<User> {
//!     Json(User { id: 1 })
//! }
//!
//! let (_, mut api) = App::new()
//!     .into_fastapi_app()
//!     .service(scope::scope("/api/v1").service(get_user))
//!     .split_for_parts();
//! ```

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

use core::fmt;
use std::future::Future;

use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::dev::{HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use fastapi::openapi::PathItem;
use fastapi::OpenApi;

use self::service_config::ServiceConfig;

pub mod scope;
pub mod service_config;

pub use scope::scope;

/// This trait is used to unify OpenAPI items collection from types implementing this trait.
pub trait OpenApiFactory {
    /// Get OpenAPI paths.
    fn paths(&self) -> fastapi::openapi::path::Paths;
    /// Collect schema reference and append them to the _`schemas`_.
    fn schemas(
        &self,
        schemas: &mut Vec<(
            String,
            fastapi::openapi::RefOr<fastapi::openapi::schema::Schema>,
        )>,
    );
}

impl<'t, T: fastapi::Path + fastapi::__dev::SchemaReferences + fastapi::__dev::Tags<'t>>
    OpenApiFactory for T
{
    fn paths(&self) -> fastapi::openapi::path::Paths {
        let methods = T::methods();

        methods
            .into_iter()
            .fold(
                fastapi::openapi::path::Paths::builder(),
                |mut builder, method| {
                    let mut operation = T::operation();
                    let other_tags = T::tags();
                    if !other_tags.is_empty() {
                        let tags = operation.tags.get_or_insert(Vec::new());
                        tags.extend(other_tags.into_iter().map(ToString::to_string));
                    };

                    let path_item = PathItem::new(method, operation);
                    builder = builder.path(T::path(), path_item);

                    builder
                },
            )
            .build()
    }

    fn schemas(
        &self,
        schemas: &mut Vec<(
            String,
            fastapi::openapi::RefOr<fastapi::openapi::schema::Schema>,
        )>,
    ) {
        <T as fastapi::__dev::SchemaReferences>::schemas(schemas);
    }
}

/// Extends [`actix_web::App`] with `fastapi` related functionality.
pub trait AppExt<T> {
    /// Convert's this [`actix_web::App`] to [`FastapiApp`].
    ///
    /// See usage from [`FastapiApp`][struct@FastapiApp]
    fn into_fastapi_app(self) -> FastapiApp<T>;
}

impl<T> AppExt<T> for actix_web::App<T> {
    fn into_fastapi_app(self) -> FastapiApp<T> {
        FastapiApp::from(self)
    }
}

/// Wrapper type for [`actix_web::App`] and [`fastapi::openapi::OpenApi`].
///
/// [`FastapiApp`] behaves exactly same way as [`actix_web::App`] but allows automatic _`schema`_ and
/// _`path`_ collection from `service(...)` calls directly or via [`ServiceConfig::service`].
///
/// It exposes typical methods from [`actix_web::App`] and provides custom [`FastapiApp::map`]
/// method to add additional configuration options to wrapper [`actix_web::App`].
///
/// This struct need be instantiated from [`actix_web::App`] by calling `.into_fastapi_app()`
/// because we do not have access to _`actix_web::App<T>`_ generic argument and the _`App`_ does
/// not provide any default implementation.
///
/// # Examples
///
/// _**Create new [`FastapiApp`] instance.**_
/// ```rust
/// # use fastapi_actix_web::{AppExt, FastapiApp};
/// # use actix_web::App;
/// let fastapi_app = App::new().into_fastapi_app();
/// ```
///
/// _**Convert `actix_web::App<T>` to `FastapiApp<T>`.**_
/// ```rust
/// # use fastapi_actix_web::{AppExt, FastapiApp};
/// # use actix_web::App;
/// let a: FastapiApp<_> = actix_web::App::new().into();
/// ```
pub struct FastapiApp<T>(actix_web::App<T>, fastapi::openapi::OpenApi);

impl<T> From<actix_web::App<T>> for FastapiApp<T> {
    fn from(value: actix_web::App<T>) -> Self {
        #[derive(OpenApi)]
        struct Api;
        FastapiApp(value, Api::openapi())
    }
}

impl<T> FastapiApp<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    /// Replace the wrapped [`fastapi::openapi::OpenApi`] with given _`openapi`_.
    ///
    /// This is useful to prepend OpenAPI doc generated with [`FastapiApp`]
    /// with content that cannot be provided directly via [`FastapiApp`].
    ///
    /// # Examples
    ///
    /// _**Replace wrapped [`fastapi::openapi::OpenApi`] with custom one.**_
    /// ```rust
    /// # use fastapi_actix_web::{AppExt, FastapiApp};
    /// # use actix_web::App;
    /// # use fastapi::OpenApi;
    /// #[derive(OpenApi)]
    /// #[openapi(info(title = "Api title"))]
    /// struct Api;
    ///
    /// let _ = actix_web::App::new().into_fastapi_app().openapi(Api::openapi());
    /// ```
    pub fn openapi(mut self, openapi: fastapi::openapi::OpenApi) -> Self {
        self.1 = openapi;

        self
    }

    /// Passthrough implementation for [`actix_web::App::app_data`].
    pub fn app_data<U: 'static>(self, data: U) -> Self {
        let app = self.0.app_data(data);
        Self(app, self.1)
    }

    /// Passthrough implementation for [`actix_web::App::data_factory`].
    pub fn data_factory<F, Out, D, E>(self, data: F) -> Self
    where
        F: Fn() -> Out + 'static,
        Out: Future<Output = Result<D, E>> + 'static,
        D: 'static,
        E: std::fmt::Debug,
    {
        let app = self.0.data_factory(data);

        Self(app, self.1)
    }

    /// Extended version of [`actix_web::App::configure`] which handles _`schema`_ and _`path`_
    /// collection from [`ServiceConfig`] into the wrapped [`fastapi::openapi::OpenApi`] instance.
    pub fn configure<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ServiceConfig),
    {
        let mut openapi = self.1;

        let app = self.0.configure(|config| {
            let mut service_config = ServiceConfig::new(config);

            f(&mut service_config);

            let paths = service_config.1.take();
            openapi.paths.merge(paths);
            let schemas = service_config.2.take();
            let components = openapi
                .components
                .get_or_insert(fastapi::openapi::Components::new());
            components.schemas.extend(schemas);
        });

        Self(app, openapi)
    }

    /// Passthrough implementation for [`actix_web::App::route`].
    pub fn route(self, path: &str, route: actix_web::Route) -> Self {
        let app = self.0.route(path, route);

        Self(app, self.1)
    }

    /// Extended version of [`actix_web::App::service`] method which handles _`schema`_ and _`path`_
    /// collection from [`HttpServiceFactory`].
    pub fn service<F>(self, factory: F) -> Self
    where
        F: HttpServiceFactory + OpenApiFactory + 'static,
    {
        let mut schemas = Vec::<(
            String,
            fastapi::openapi::RefOr<fastapi::openapi::schema::Schema>,
        )>::new();

        factory.schemas(&mut schemas);
        let paths = factory.paths();

        let mut openapi = self.1;

        openapi.paths.merge(paths);
        let components = openapi
            .components
            .get_or_insert(fastapi::openapi::Components::new());
        components.schemas.extend(schemas);

        let app = self.0.service(factory);

        Self(app, openapi)
    }

    /// Helper method to serve wrapped [`fastapi::openapi::OpenApi`] via [`HttpServiceFactory`].
    ///
    /// This method functions as a convenience to serve the wrapped OpenAPI spec alternatively to
    /// first call [`FastapiApp::split_for_parts`] and then calling [`actix_web::App::service`].
    pub fn openapi_service<O, F>(self, factory: F) -> Self
    where
        F: FnOnce(fastapi::openapi::OpenApi) -> O,
        O: HttpServiceFactory + 'static,
    {
        let service = factory(self.1.clone());
        let app = self.0.service(service);
        Self(app, self.1)
    }

    /// Passthrough implementation for [`actix_web::App::default_service`].
    pub fn default_service<F, U>(self, svc: F) -> Self
    where
        F: IntoServiceFactory<U, ServiceRequest>,
        U: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error>
            + 'static,
        U::InitError: fmt::Debug,
    {
        Self(self.0.default_service(svc), self.1)
    }

    /// Passthrough implementation for [`actix_web::App::external_resource`].
    pub fn external_resource<N, U>(self, name: N, url: U) -> Self
    where
        N: AsRef<str>,
        U: AsRef<str>,
    {
        Self(self.0.external_resource(name, url), self.1)
    }

    /// Convenience method to add custom configuration to [`actix_web::App`] that is not directly
    /// exposed via [`FastapiApp`]. This could for example be adding middlewares.
    ///
    /// # Examples
    ///
    /// _**Add middleware via `map` method.**_
    ///
    /// ```rust
    /// # use fastapi_actix_web::{AppExt, FastapiApp};
    /// # use actix_web::App;
    /// # use actix_service::Service;
    /// # use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
    ///  let _ = App::new()
    ///     .into_fastapi_app()
    ///     .map(|app| {
    ///            app.wrap_fn(|req, srv| {
    ///                let fut = srv.call(req);
    ///                async {
    ///                    let mut res = fut.await?;
    ///                    res.headers_mut()
    ///                        .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    ///                    Ok(res)
    ///                }
    ///            })
    ///        });
    /// ```
    pub fn map<
        F: FnOnce(actix_web::App<T>) -> actix_web::App<NF>,
        NF: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
    >(
        self,
        op: F,
    ) -> FastapiApp<NF> {
        let app = op(self.0);
        FastapiApp(app, self.1)
    }

    /// Split this [`FastapiApp`] into parts returning tuple of [`actix_web::App`] and
    /// [`fastapi::openapi::OpenApi`] of this instance.
    pub fn split_for_parts(self) -> (actix_web::App<T>, fastapi::openapi::OpenApi) {
        (self.0, self.1)
    }

    /// Converts this [`FastapiApp`] into the wrapped [`actix_web::App`].
    pub fn into_app(self) -> actix_web::App<T> {
        self.0
    }
}

impl<T> From<FastapiApp<T>> for actix_web::App<T> {
    fn from(value: FastapiApp<T>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use actix_service::Service;
    use actix_web::guard::{Get, Guard};
    use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
    use actix_web::web::{self, Data};
    use actix_web::{get, App, HttpRequest, HttpResponse};
    use fastapi::ToSchema;

    use super::*;

    #[derive(ToSchema)]
    struct Value12 {
        v: String,
    }

    #[derive(ToSchema)]
    struct Value2(i32);

    #[derive(ToSchema)]
    struct Value1 {
        bar: Value2,
    }

    #[derive(ToSchema)]
    struct ValueValue {
        value: i32,
    }

    #[fastapi::path(responses(
        (status = 200, body = ValueValue)
    ))]
    #[get("/handler2")]
    async fn handler2() -> &'static str {
        "this is message 2"
    }

    #[fastapi::path(responses(
        (status = 200, body = Value12)
    ))]
    #[get("/handler")]
    async fn handler() -> &'static str {
        "this is message"
    }

    #[fastapi::path(responses(
        (status = 200, body = Value1)
    ))]
    #[get("/handler3")]
    async fn handler3() -> &'static str {
        "this is message 3"
    }

    mod inner {
        use actix_web::get;
        use actix_web::web::Data;
        use fastapi::ToSchema;

        #[derive(ToSchema)]
        struct Bar(i32);

        #[derive(ToSchema)]
        struct Foobar {
            bar: Bar,
        }

        #[fastapi::path(responses(
            (status = 200, body = Foobar)
        ))]
        #[get("/inner_handler")]
        pub async fn inner_handler(_: Data<String>) -> &'static str {
            "this is message"
        }

        #[fastapi::path()]
        #[get("/inner_handler3")]
        pub async fn inner_handler3(_: Data<String>) -> &'static str {
            "this is message 3"
        }
    }

    #[get("/normal_service")]
    async fn normal_service() -> &'static str {
        "str"
    }

    #[test]
    fn test_app_generate_correct_openapi() {
        fn config(cfg: &mut service_config::ServiceConfig) {
            cfg.service(handler3)
                .map(|config| config.service(normal_service));
        }

        let (_, mut api) = App::new()
            .into_fastapi_app()
            .service(handler)
            .configure(config)
            .service(scope::scope("/path-prefix").service(handler2).map(|scope| {
                let s = scope.wrap_fn(|req, srv| {
                    let fut = srv.call(req);
                    async {
                        let mut res = fut.await?;
                        res.headers_mut()
                            .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
                        Ok(res)
                    }
                });

                s
            }))
            .service(scope::scope("/api/v1/inner").configure(|cfg| {
                cfg.service(inner::inner_handler)
                    .service(inner::inner_handler3)
                    .app_data(Data::new(String::new()));
            }))
            .split_for_parts();
        api.info = fastapi::openapi::info::Info::new("title", "version");
        let json = api.to_pretty_json().expect("OpenAPI is JSON serializable");
        println!("{json}");

        let expected = include_str!("../testdata/app_generated_openapi");
        assert_eq!(json.trim(), expected.trim());
    }
}
