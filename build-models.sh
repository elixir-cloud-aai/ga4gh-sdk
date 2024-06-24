#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

get_git_repo_name() {
    # Extract the URL of the remote "origin"
    url=$(git config --get remote.origin.url)

    # Extract the repository name from the URL
    repo_name=$(basename -s .git "$url")

    echo "$repo_name"
}

repo_name=$(get_git_repo_name)
if [ "$repo_name" != "ga4gh-sdk" ]; then
    echo "This script must be run from the 'ga4gh-sdk' repository."
    exit 1
fi

cd $(git rev-parse --show-toplevel)
SCRIPT_DIR="$(pwd)"

generate_openapi_models() {
    # Parameters
    OPENAPI_SPEC_PATH="$1"
    API_NAME="$2"
    DESTINATION_DIR="$3"

    # Define the temporary output directory for the OpenAPI generator
    TEMP_OUTPUT_DIR=$(mktemp -d)

    # Remove the temporary directory at the end of the script
    trap 'rm -rf "$TEMP_OUTPUT_DIR"' EXIT

    # Run the OpenAPI generator CLI
    npx openapi-generator-cli generate -g rust \
        -i "$OPENAPI_SPEC_PATH" \
        -o "$TEMP_OUTPUT_DIR" \
        --additional-properties=useSingleRequestParameter=true 
        #--skip-validate-spec
        # --global-property models,modelDocs=false,apiDocs=false,apiTests=false,modelTests=false \
    #,packageName=$API_NAME

    # Check if the generation was successful
    if [ $? -ne 0 ]; then
        echo "OpenAPI generation failed. Check the verbose output for details."
        exit 1
    fi

    # Remove the openapitools.json file
    rm -f ./openapitools.json
    
    echo "TEMP_OUTPUT_DIR is $TEMP_OUTPUT_DIR"
    # Modify the import statements in each generated file
    for file in $(find "$TEMP_OUTPUT_DIR" -name '*.rs'); do
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS (BSD) sed syntax
            sed -i '' "s/use crate::models;/#![allow(unused_imports)]\nuse crate::$API_NAME::models;/" "$file"
        else
            # Linux (GNU) sed syntax
            sed -i "s/use crate::models;/#![allow(unused_imports)]\nuse crate::$API_NAME::models;/" "$file"
        fi
    done

    rm -rf "$DESTINATION_DIR/models"
    mkdir -p "$DESTINATION_DIR"
    cp -r "$TEMP_OUTPUT_DIR/src/models" "$DESTINATION_DIR"

    echo "OpenAPI generation complete. Models copied to $DESTINATION_DIR"
}

# Check if OpenAPI Generator CLI is installed
if ! npx openapi-generator-cli version > /dev/null 2>&1; then
    # Install OpenAPI Generator CLI locally
    npm install -g @openapitools/openapi-generator-cli
fi

generate_openapi_models \
    "https://raw.githubusercontent.com/ga4gh-discovery/ga4gh-service-info/develop/service-info.yaml" \
    "serviceinfo" "$SCRIPT_DIR/lib/src/serviceinfo/"

generate_openapi_models \
    "https://raw.githubusercontent.com/ga4gh/task-execution-schemas/develop/openapi/task_execution_service.openapi.yaml" \
    "tes" "$SCRIPT_DIR/lib/src/tes/"
