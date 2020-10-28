# GRIP API
Web API backend for CAIDA Global Routing Intelligence Platform Project.

## Configuration Files

- Rocket.toml: configuring webservice basics
- Config.toml: general configurations

## Installation

### Environment Setup

1. Make sure to first install the service file(s) under `systemd` directory.

2. Install Rust nightly environment under the home directory of the user who
compiles the code:

``` sh
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2020-01-01 -y
```

Note that, running the API does *not* require compilation or rust environment.

### Compile and Install

`sudo make install` will:
- build the rust code
- install two binary applications

`sudo make restart` will:
- restart the API service
