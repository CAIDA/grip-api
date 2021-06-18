LABEL maintainer="Mingwei Zhang <mingwei@caida.org>"
# select build image
FROM rust:1.52 as build

# create a new empty shell project
RUN USER=root cargo new --bin my_project
WORKDIR /my_project

# change toolchain to nightly for rocket
RUN rustup default nightly

# build for release
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN DEBIAN=NONINTERACTIVE apt update; apt install -y libssl-dev libpq-dev ca-certificates tzdata; rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /my_project/target/release/grip-api /usr/local/bin/grip-api
COPY --from=build /my_project/target/release/grip-cli /usr/local/bin/grip-cli

# set the startup command to run your binary
CMD ["grip-api"]

