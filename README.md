# A Generic SDK and CLI for GA4GH API services

## Building

First, clone the repository, and then run the following command to auto-generate models using OpenAPI specifications:
```
bash ./build-models.sh
```

To build the project:
```
cargo build
```

## Running the tests

Before running the tests, you need to install Funnel, a task execution system that is compatible with the GA4GH TES API. Follow the instructions in the [Funnel Developer's Guide](https://ohsu-comp-bio.github.io/funnel/docs/development/developers/) to install Funnel.

Once you have installed Funnel, you can run the tests with (it will automatically run Funnel as well):

```
bash ./run-test.sh
```

To test out the CI/CD workflow locally, install `act` and run the following command:
```
act -j build --container-architecture linux/amd64 -P ubuntu-latest=ubuntu:20.04 --reuse
```