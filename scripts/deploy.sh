#!/usr/bin/env bash

set -e

echo "Fetching latest GitHub release for project"
RESPONSE=$(curl -s -L -H "Accept: application/vnd.github+json" -H "X-GitHub-Api-Version: 2022-11-28" https://api.github.com/repos/lunfel/voxel/releases/latest)

TAG_NAME=$(echo $RESPONSE | jq -r '.tag_name')
WASM_ARTEFACT=$(echo $RESPONSE | jq '.assets[] | first(select(.name | match(".wasm$")))')
WASM_NAME=$(echo $WASM_ARTEFACT | jq -r '.name')
WASM_URL=$(echo $WASM_ARTEFACT | jq -r '.browser_download_url')
JS_ARTEFACT=$(echo $RESPONSE | jq '.assets[] | first(select(.name | match(".js$")))')
JS_NAME=$(echo $JS_ARTEFACT | jq -r '.name')
JS_URL=$(echo $JS_ARTEFACT | jq -r '.browser_download_url')

mkdir -p releases/$TAG_NAME
echo "Downloading source code for assets"
curl -s -L https://github.com/lunfel/voxel/archive/refs/tags/$TAG_NAME.tar.gz | tar -xz --strip-components=1 -C releases/$TAG_NAME

echo "Downloading $WASM_NAME ($WASM_URL)"
curl -s -L -O --output-dir releases/$TAG_NAME "$WASM_URL"
echo "Downloading $JS_NAME ($JS_URL)"
curl -s -L -O --output-dir releases/$TAG_NAME "$JS_URL"

echo "$RESPONSE" > releases/$TAG_NAME/latest.json

echo "Pushing to web server..."
rsync -av releases/$TAG_NAME/assets releases/$TAG_NAME/$WASM_NAME releases/$TAG_NAME/$JS_NAME releases/$TAG_NAME/latest.json web/index.html voxel:/var/www/voxel/
echo "Done!"