## Motivation
You will only need to use the script to build models when any of the GA4GH API's is updated. The models are being used to return the result of any API call in the correct format, with the correct structs


## Usage

First, clone the repository, and then update the specs of the APIs you want to change in /utils/build-models.sh. Then, run the following command to automatically generate models using OpenAPI specifications: 
```
bash ./utils/build-models.sh
```