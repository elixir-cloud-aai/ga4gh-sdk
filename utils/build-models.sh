#!/bin/bash

# Define the list of OpenAPI specifications and their corresponding API names and destination directories
declare -A SPECS
SPECS["serviceinfo"]="https://raw.githubusercontent.com/ga4gh-discovery/ga4gh-service-info/develop/service-info.yaml lib/src/serviceinfo/"
SPECS["tes"]="https://raw.githubusercontent.com/ga4gh/task-execution-schemas/develop/openapi/task_execution_service.openapi.yaml lib/src/tes/"

# Directory of the build script
BUILD_SCRIPT="./utils/script.sh"

# Iterate over the specifications and call the build script for each
for API_NAME in "${!SPECS[@]}"; do
    SPEC_URL=$(echo ${SPECS[$API_NAME]} | awk '{print $1}')
    DESTINATION_DIR=$(echo ${SPECS[$API_NAME]} | awk '{print $2}')
    
    echo "Generating models for $API_NAME from $SPEC_URL into $DESTINATION_DIR"
    $BUILD_SCRIPT "$SPEC_URL" "$API_NAME" "$DESTINATION_DIR"
done
