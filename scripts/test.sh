#!/bin/bash
# shellcheck disable=SC2086

set -e

: "${CARGO:=cargo}"
: "${CARGO_COMMAND:=test}"

crates="${1:-fastapi fastapi-gen fastapi-swagger-ui fastapi-redoc fastapi-rapidoc fastapi-scalar fastapi-axum fastapi-config fastapi-actix-web}"

for crate in $crates; do
    echo "Testing crate: $crate..."

    # Always test the crate itself first, without any features added.
    if [[ "$crate" != "fastapi-gen" ]]; then
        $CARGO ${CARGO_COMMAND} -p $crate
    fi

    if [[ "$crate" == "fastapi" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi --features openapi_extensions,preserve_order,preserve_path_order,debug,macros
    elif [[ "$crate" == "fastapi-gen" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-gen --features fastapi/actix_extras,chrono,decimal,fastapi/uuid,uuid,fastapi/ulid,ulid,fastapi/url,url,fastapi/time,time,fastapi/repr,fastapi/smallvec,smallvec,rc_schema,fastapi/rc_schema,fastapi/macros
        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test schema_derive_test --features decimal_float,fastapi/macros

        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_auto_into_responses --features auto_into_responses,fastapi/uuid,uuid,fastapi/macros
        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_actix --test path_parameter_derive_actix --features actix_extras,fastapi/uuid,uuid,fastapi/chrono,chrono,fastapi/time,time,fastapi/macros
        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_auto_into_responses_actix --features actix_extras,fastapi/auto_into_responses,fastapi/uuid,uuid,fastapi/macros

        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_rocket --features rocket_extras,fastapi/macros

        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_axum_test --features axum_extras,fastapi/macros
        $CARGO ${CARGO_COMMAND} -p fastapi-gen --test path_derive_auto_into_responses_axum --features axum_extras,fastapi/auto_into_responses,fastapi/macros
    elif [[ "$crate" == "fastapi-swagger-ui" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-swagger-ui --features actix-web,rocket,axum,fastapi/macros
    elif [[ "$crate" == "fastapi-redoc" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-redoc --features actix-web,rocket,axum,fastapi/macros
    elif [[ "$crate" == "fastapi-rapidoc" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-rapidoc --features actix-web,rocket,axum,fastapi/macros
    elif [[ "$crate" == "fastapi-scalar" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-scalar --features actix-web,rocket,axum,fastapi/macros
    elif [[ "$crate" == "fastapi-axum" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-axum --features debug,fastapi/debug,fastapi/macros
    elif [[ "$crate" == "fastapi-config" ]]; then
        pushd fastapi-config/config-test-crate/
        $CARGO ${CARGO_COMMAND}
        popd
    elif [[ "$crate" == "fastapi-actix-web" ]]; then
        $CARGO ${CARGO_COMMAND} -p fastapi-actix-web
    fi
done
