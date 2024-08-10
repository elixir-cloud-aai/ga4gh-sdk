## Motivation
You will only need to use the script to build models when any of the GA4GH API's openapi specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs

## Prerequisites
There are no particular prerequisites for running this script except giving permission to build-models.sh to run script.sh, which can be done by:
```
sudo chmod +x ./utils/script.sh
```

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
