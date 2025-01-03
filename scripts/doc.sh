#!/bin/bash

# Generate fastapi workspace docs

cargo +nightly doc -Z unstable-options --workspace --no-deps \
    --features actix_extras,openapi_extensions,yaml,uuid,ulid,url,non_strict_integers,actix-web,axum,rocket,macros,config \
    --config 'build.rustdocflags = ["--cfg", "doc_cfg"]'
