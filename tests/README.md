# A folder for adding the files being used in unit tests


sample.tes: This is a sample file, taken from the [funnel docs](https://ohsu-comp-bio.github.io/funnel/docs/tasks/), and this file is being used in the file lib/src/tes/mod.rs


grape.tes: a sample file containing JSON task data for the GA4GH [Task Execution Service](https://github.com/ga4gh/task-execution-schemas), which can be used in the file lib/src/tes/mod.rs instead of sample.tes. Notably, it has placeholders like "${AWS_ACCESS_KEY_ID}" which is out of the standard and implies implementing a pre-processor, might be useful to note and implement in future as it avoids storing credentials in such .tes files
