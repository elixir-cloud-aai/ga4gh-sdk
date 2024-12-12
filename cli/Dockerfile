FROM ubuntu:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates pkg-config cargo git nano sudo \
    libtss2-dev libssl-dev 

RUN useradd -ms /bin/bash researcher
WORKDIR /home/researcher/

RUN git clone https://github.com/elixir-cloud-aai/ga4gh-sdk
RUN cd ga4gh-sdk && git switch feat/cli-dockerfile && git pull
RUN cd ga4gh-sdk/cli && cargo build --release
RUN ln -s /home/researcher/ga4gh-sdk/target/release/cli /usr/local/bin/ga4gh-cli

RUN chown -R researcher:researcher /home/researcher
USER researcher

CMD ["bash"]
