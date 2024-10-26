## Motivation
You will only need to use the script to build models when any of the GA4GH API's [OpenAPI](https://www.openapis.org/) specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs.

## Prerequisites and Dependencies:
1. OpenAPI Generator CLI: Make sure the directory `~/bin/openapitools` is empty, as OpenAPI is being installed in this script, which might conflict with your existing directory. Then run the following script to install OpenAPI generator CLI.
    ```sh
    mkdir -p ~/bin/openapitools
    curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli
    chmod u+x ~/bin/openapitools/openapi-generator-cli
    export PATH=$PATH:~/bin/openapitools/
    ```
For more information, and for looking at alternate ways to install OpenAPI Generator CLI, see the documentation at https://openapi-generator.tech/docs/installation/ 

2. Java Development Kit (JDK)
    The OpenAPI generator CLI requires Java to run. Ensure that you have the JDK installed on your system.
    You can install the JDK using the following command:
    - On Ubuntu/Debian:
        ```sh
        sudo apt-get update
        sudo apt-get install default-jdk
        ```
    - On Fedora:
        ```sh
        sudo dnf install java-11-openjdk
        ```
    - On macOS (using Homebrew):
        ```sh
        brew install openjdk@11
        ```

## Usage

### Option 1
1. Clone the repository:
    ```sh
    git clone https://github.com/elixir-cloud-aai/ga4gh-sdk.git
    ```
2. Update the specs of the APIs you want to change in `./utils/build-models.sh`:
    ```sh
    declare -A SPECS
    SPECS["serviceinfo"]="new serviceinfo specs"
    SPECS["tes"]="new tes openapi specs"
    ```
3. Run the following command to automatically generate models using OpenAPI specifications:
    ```sh
    bash ./utils/build_models_wrapper.sh
    ```

### Option 2
To run the script, use the following command:
```sh
./build_models.sh <OPENAPI_SPEC_PATH> <API_NAME> <DESTINATION_DIR>
