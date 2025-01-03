#![allow(dead_code)]

use fastapi::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        server::{ServerBuilder, ServerVariableBuilder},
    },
    Modify, OpenApi, ToSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"name": "bob the cat", "id": 1}))]
struct Pet {
    id: u64,
    name: String,
    age: Option<i32>,
}

mod pet_api {
    use super::*;

    /// Get pet by id
    ///
    /// Get pet from database by pet database id
    #[fastapi::path(
        get,
        path = "/pets/{id}",
        responses(
            (status = 200, description = "Pet found successfully", body = Pet),
            (status = 404, description = "Pet was not found")
        ),
        params(
            ("id" = u64, Path, description = "Pet database id to get Pet for"),
        ),
        security(
            (),
            ("my_auth1" = ["read:items", "edit:items"], "my_auth2" = ["read:items"]),
            ("token_jwt" = [])
        )
    )]
    #[allow(unused)]
    async fn get_pet_by_id(pet_id: u64) -> Pet {
        Pet {
            id: pet_id,
            age: None,
            name: "lightning".to_string(),
        }
    }
}

#[derive(Default, OpenApi)]
#[openapi(
    paths(pet_api::get_pet_by_id),
    components(schemas(Pet, C<A, B>, C<B, A>)),
    modifiers(&Foo),
    security(
        (),
        ("my_auth1" = ["read:items", "edit:items"], "my_auth2" = ["read:items"]),
        ("token_jwt" = [])
    )
)]
struct ApiDoc;

#[derive(Deserialize, Serialize, ToSchema)]
struct A {
    a: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
struct B {
    b: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
struct C<T, R> {
    field_1: R,
    field_2: T,
}

#[derive(Debug, Serialize)]
struct Foo;

#[derive(Debug, Serialize)]
struct FooResources;

impl Modify for Foo {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "token_jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }

        openapi.servers = Some(vec![ServerBuilder::new()
            .url("/api/bar/{username}")
            .description(Some("this is description of the server"))
            .parameter(
                "username",
                ServerVariableBuilder::new()
                    .default_value("the_user")
                    .description(Some("this is user")),
            )
            .build()]);
    }
}

#[test]
#[cfg(feature = "yaml")]
fn stable_yaml() {
    let left = ApiDoc::openapi().to_yaml().unwrap();
    let right = ApiDoc::openapi().to_yaml().unwrap();
    assert_eq!(left, right);
}

#[test]
fn stable_json() {
    let left = ApiDoc::openapi().to_json().unwrap();
    let right = ApiDoc::openapi().to_json().unwrap();
    assert_eq!(left, right);
}
