## Motivation
You will only need to use the script to build models when any of the GA4GH API's openapi specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs

## Prerequisites and Dependencies:
1. The script.sh should have permission to run, which can be done by:
```
sudo chmod +x ./utils/script.sh
```
2.  OpenAPI Generator CLI: Make sure the directory `~/bin/openapitools` is empty, as OpenAPI is being installed in this script, which might conflict with your exisiting directory.

3. Java Development Kit (JDK)
    The OpenAPI generator CLI requires Java to run. Ensure that you have the JDK installed on your system.
    You can install the JDK using the following command:
    On Ubuntu/Debian:
    sudo apt-get update
    sudo apt-get install default-jdk

    On Fedora:
    sudo dnf install java-11-openjdk

    On macOS (using Homebrew):
    brew install openjdk@11

## Usage

# Option 1
1. Clone the repository
```
git clone https://github.com/elixir-cloud-aai/ga4gh-sdk.git
```
2. Update the specs of the APIs you want to change in `/utils/build-models.sh` over here:

```
declare -A SPECS
SPECS["serviceinfo"]="new serviceinfo specs"
SPECS["tes"]="new tes openapi specs"
```

3. Run the following command to automatically generate models using OpenAPI specifications: 
```
bash ./utils/build-models.sh
```

# Option 2
# -----
# To run the script, use the following command:
# ./script.sh <OPENAPI_SPEC_PATH> <API_NAME> <DESTINATION_DIR>
#
# Example:
# ./script.sh https://example.com/path/to/openapi.yaml myapi lib/src/clients/myapi/
#
# Parameters:
# - OPENAPI_SPEC_PATH: The URL or file path to the OpenAPI specification.
# - API_NAME: The name of the API.
# - DESTINATION_DIR: The directory where the generated models should be copied.
#
# The script will generate Rust models from the OpenAPI specification and place them in the specified destination directory.
