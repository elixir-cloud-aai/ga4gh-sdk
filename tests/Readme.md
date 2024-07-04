# A folder for adding the files being used in unit tests

grape.tes: a sample file containing JSON task data for the GA4GH [Task Execution Service](https://github.com/ga4gh/task-execution-schemas) in the file lib/src/tes/mod.rs. Notably, it has placeholders like "${AWS_ACCESS_KEY_ID}" which is out of the standard and implies implementing a pre-processor, might be useful to note and implement in future as it avoids storing credentials in such .tes files
