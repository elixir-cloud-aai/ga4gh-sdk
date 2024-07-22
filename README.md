# GA4GH SDK

## Synopsis

A Generic SDK and CLI for GA4GH API services

## Basic usage

## Installation

## Building

First, clone the repository, and then run the following command to automatically generate models using OpenAPI specifications:
```
bash ./build-models.sh
```

To build the project:
```
cargo build
```

## Running the tests

Before running the tests, you need to install Funnel, a task execution system that is compatible with the GA4GH TES API. Follow the instructions in the Funnel Developer's Guide to install Funnel: https://ohsu-comp-bio.github.io/funnel/docs/development/developers/. Note that the link may change over time, so refer to the official Funnel repository or website for the most up-to-date information.

Once you have installed Funnel, you can run the tests. This will automatically run Funnel as well:

```
bash ./run-tests.sh
```
or, you can run using:
```
cargo nextest run
```
For checking the unit coverage, you can run:
```
cargo llvm-cov nextest
```

To test the CI/CD workflow locally, install `act` and run the following command:
```
act -j build --container-architecture linux/amd64 -P ubuntu-latest=ubuntu:24.04 --reuse
# Note: The specified Ubuntu version (24.04) may change in the future. 
# Please check for the latest version when running this command.
```

## Versioning

## Contributing

This project is a community effort and lives off your contributions, be it in
the form of bug reports, feature requests, discussions, ideas, fixes, or other
code changes. Please read [these guidelines][docs-contributing] if you want to
contribute. And please mind the [code of conduct][docs-coc] for all
interactions with the community.

## License

This project is covered by the [Apache License 2.0][badge-url-license] also
[shipped with this repository][docs-license].

## Contact
