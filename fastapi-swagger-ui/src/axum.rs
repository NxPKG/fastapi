#![cfg(feature = "axum")]

use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing, Extension, Json, Router,
};

use crate::{ApiDoc, Config, SwaggerUi, Url};

impl<S> From<SwaggerUi> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn from(swagger_ui: SwaggerUi) -> Self {
        let urls_capacity = swagger_ui.urls.len();
        let external_urls_capacity = swagger_ui.external_urls.len();

        let (router, urls) = swagger_ui.urls.into_iter().fold(
            (
                Router::<S>::new(),
                Vec::<Url>::with_capacity(urls_capacity + external_urls_capacity),
            ),
            |router_and_urls, (url, openapi)| {
                add_api_doc_to_urls(router_and_urls, (url, ApiDoc::Fastapi(openapi)))
            },
        );
        let (router, urls) = swagger_ui.external_urls.into_iter().fold(
            (router, urls),
            |router_and_urls, (url, openapi)| {
                add_api_doc_to_urls(router_and_urls, (url, ApiDoc::Value(openapi)))
            },
        );

        let config = if let Some(config) = swagger_ui.config {
            if config.url.is_some() || !config.urls.is_empty() {
                config
            } else {
                config.configure_defaults(urls)
            }
        } else {
            Config::new(urls)
        };

        let handler = routing::get(serve_swagger_ui).layer(Extension(Arc::new(config)));
        let path: &str = swagger_ui.path.as_ref();

        if path == "/" {
            router
                .route(path, handler.clone())
                .route(&format!("{}*rest", path), handler)
        } else {
            let path = if path.ends_with('/') {
                &path[..path.len() - 1]
            } else {
                path
            };
            debug_assert!(!path.is_empty());

            let slash_path = format!("{}/", path);
            router
                .route(
                    path,
                    routing::get(|| async move { axum::response::Redirect::to(&slash_path) }),
                )
                .route(&format!("{}/", path), handler.clone())
                .route(&format!("{}/*rest", path), handler)
        }
    }
}

fn add_api_doc_to_urls<S>(
    router_and_urls: (Router<S>, Vec<Url<'static>>),
    url: (Url<'static>, ApiDoc),
) -> (Router<S>, Vec<Url<'static>>)
where
    S: Clone + Send + Sync + 'static,
{
    let (router, mut urls) = router_and_urls;
    let (url, openapi) = url;
    (
        router.route(
            url.url.as_ref(),
            routing::get(move || async { Json(openapi) }),
        ),
        {
            urls.push(url);
            urls
        },
    )
}

async fn serve_swagger_ui(
    path: Option<Path<String>>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    let tail = match path.as_ref() {
        Some(tail) => tail,
        None => "",
    };

    match super::serve(tail, state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn mount_onto_root() {
        let app = Router::<()>::from(SwaggerUi::new("/"));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;
        response.assert_status_ok();
        let response = server.get("/swagger-ui.css").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn mount_onto_path_ends_with_slash() {
        let app = Router::<()>::from(SwaggerUi::new("/swagger-ui/"));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/swagger-ui").await;
        response.assert_status_see_other();
        let response = server.get("/swagger-ui/").await;
        response.assert_status_ok();
        let response = server.get("/swagger-ui/swagger-ui.css").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn mount_onto_path_not_end_with_slash() {
        let app = Router::<()>::from(SwaggerUi::new("/swagger-ui"));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/swagger-ui").await;
        response.assert_status_see_other();
        let response = server.get("/swagger-ui/").await;
        response.assert_status_ok();
        let response = server.get("/swagger-ui/swagger-ui.css").await;
        response.assert_status_ok();
    }
}
