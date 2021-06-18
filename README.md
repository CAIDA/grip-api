# GRIP API

Web API backend for CAIDA Global Routing Intelligence Platform Project.

## Configuration Files

- Rocket.toml: configuring webservice basics
- Config.toml: general configurations

## Installation with Docker

### Build container
Checkout the repository and run the following command:

``` sh
docker-compose build
```

It should build a docker container named `grip-api`. 

### Run as commands

The default command of the container is to run `grip-api`.

``` sh
docker run --rm -it grip-api
```

It also supports running the `grip-cli` command by replace the default command:

``` sh
docker run --rm -it grip-api grip-cli --help
```

### Run as service

You can also run the API as a service:

``` sh
docker-compose up
```

To run the service in the background:

``` sh
docker-compose up -d
```

It should display something like this:

``` sh
mingwei@MacBook-Pro:~/git/caida-git/bgphijacks-dashboard$ docker-compose up
Creating bgphijacks-dashboard_grip-api_1 ... done
Attaching to bgphijacks-dashboard_grip-api_1
grip-api_1  | Configured for production.
grip-api_1  |     => address: 0.0.0.0
grip-api_1  |     => port: 8000
grip-api_1  |     => log: critical
grip-api_1  |     => workers: 8
grip-api_1  |     => secret key: generated
grip-api_1  |     => limits: forms = 32KiB
grip-api_1  |     => keep-alive: 5s
grip-api_1  |     => read timeout: 5s
grip-api_1  |     => write timeout: 5s
grip-api_1  |     => tls: disabled
grip-api_1  | Warning: environment is 'production', but no `secret_key` is configured
grip-api_1  | Rocket has launched from http://0.0.0.0:8000
```

You can then go to your browser and go to http://0.0.0.0:8000/json/tags and verify if it's working.

You can also change thee listening port by revising the `ports` field in `docker-compose.yaml` file.

## Installation From Source

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
