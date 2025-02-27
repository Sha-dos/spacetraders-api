#!/bin/bash
CMD="npx @openapitools/openapi-generator-cli"

rm -rf api-docs/
git clone https://github.com/SpaceTradersAPI/api-docs.git
cd api-docs
git checkout a2af2dfc59c70b27b81e72c9b4b5576da3ce57b7
cd ..

npx @openapitools/openapi-generator-cli generate \
 -i api-docs/reference/SpaceTraders.json \
 -g rust --skip-validate-spec \
 --additional-properties=supportAsync=true \
 --additional-properties=supportMiddleware=true

cargo fmt
