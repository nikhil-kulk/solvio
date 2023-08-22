#!/usr/bin/env bash

set -euo pipefail

STATIC_DIR=${STATIC_DIR:-"./static"}
OPENAPI_FILE=${OPENAPI_DIR:-"./docs/redoc/master/openapi.json"}

# Download `dist.zip` from the latest release of https://github.com/solvio/solvio-web-ui and unzip given folder

# Get latest dist.zip, assume jq is installed
DOWNLOAD_LINK=$(curl --silent "https://api.github.com/repos/solvio/solvio-web-ui/releases/latest" | jq -r '.assets[] | select(.name=="dist-solvio.zip") | .browser_download_url')

wget -O dist-solvio.zip $DOWNLOAD_LINK

rm -rf "${STATIC_DIR}/"*
unzip -o dist-solvio.zip -d "${STATIC_DIR}"
rm dist-solvio.zip
cp -r "${STATIC_DIR}/dist/"* "${STATIC_DIR}"
rm -rf "${STATIC_DIR}/dist"

cp "${OPENAPI_FILE}" "${STATIC_DIR}/openapi.json"
