## Motivation
You will only need to use the script to build models when any of the GA4GH API's openapi specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs


## Usage

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
