#!/bin/bash
#
# Update Swagger UI version

set -eu -o pipefail

version="${1:-""}"
if [ -z "$version" ]; then
    echo "Missing 'version' argument from command, run as $0 <version>" >&2 && exit 1
fi
zip_name="v$version.zip"

curl -sSL -o "$zip_name" "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v$version.zip"

echo "Update vendored Swagger UI"
mv "$zip_name" ./fastapi-swagger-ui-vendored/res/
sed -i "s|version: \`.*\`|version: \`$version\`|" ./fastapi-swagger-ui-vendored/README.md
sed -i "s|version: \`.*\`|version: \`$version\`|" ./fastapi-swagger-ui-vendored/src/lib.rs
sed -i "s|res/v.*\.zip|res/v$version.zip|" ./fastapi-swagger-ui-vendored/src/lib.rs

echo "Update fastapi-swagger-ui Swagger UI version"
sed -i "s|tags/v.*>|tags/v$version.zip>|" ./fastapi-swagger-ui/README.md
sed -i "s|tags/v.*>|tags/v$version.zip>|" ./fastapi-swagger-ui/src/lib.rs
sed -i "s|tags/v.*\.zip|tags/v$version.zip|" ./fastapi-swagger-ui/build.rs
