#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e


# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Path to the OpenAPI spec file
OPENAPI_SPEC_PATH="$SCRIPT_DIR/openapi/swagger.yaml"

# Define the temporary output directory for the OpenAPI generator
TEMP_OUTPUT_DIR="$SCRIPT_DIR/tmp"
    
# Define the destination directory in your main repository
DESTINATION_DIR="$SCRIPT_DIR"

# Remove the temporary output directory if it exists
rm -rf $TEMP_OUTPUT_DIR

# Install OpenAPI Generator CLI locally
npm install @openapitools/openapi-generator-cli

# Run the OpenAPI generator CLI
npx openapi-generator-cli generate -g rust \
-i "$OPENAPI_SPEC_PATH" \
-o "$TEMP_OUTPUT_DIR" \
--additional-properties=useSingleRequestParameter=true

# Check if the generation was successful
if [ $? -eq 0 ]; then
    # Copy the models folder from the generated code to the main repository
    cp -r "$TEMP_OUTPUT_DIR/src/models" "$DESTINATION_DIR"
    
    # Clean up the temporary output directory
    rm -rf $TEMP_OUTPUT_DIR
    
    echo "OpenAPI generation complete. Models copied to $DESTINATION_DIR"
else
    echo "OpenAPI generation failed. Check the verbose output for details."
    exit 1
fi