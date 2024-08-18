## Motivation
You will only need to use the script to build models when any of the GA4GH API's OpenAPI specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs.

## Prerequisites and Dependencies:
1. The `script.sh` should have permission to run, which can be done by:
    ```sh
    sudo chmod +x ./utils/script.sh
    ```
2. OpenAPI Generator CLI: Make sure the directory `~/bin/openapitools` is empty, as OpenAPI is being installed in this script, which might conflict with your existing directory.

3. Java Development Kit (JDK)
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
2. Update the specs of the APIs you want to change in [`/utils/build-models.sh`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Faarav%2Fdev%2Fga4gh-sdk%2Futils%2Fbuild-models.sh%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "/home/aarav/dev/ga4gh-sdk/utils/build-models.sh"):
    ```sh
    declare -A SPECS
    SPECS["serviceinfo"]="new serviceinfo specs"
    SPECS["tes"]="new tes openapi specs"
    ```
3. Run the following command to automatically generate models using OpenAPI specifications:
    ```sh
    bash ./utils/build-models.sh
    ```

### Option 2
To run the script, use the following command:
```sh
./script.sh <OPENAPI_SPEC_PATH> <API_NAME> <DESTINATION_DIR>
