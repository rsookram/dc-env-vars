# dc-env-vars

Outputs the environment variables specified in a `docker-compose.yml` file for
use with setting in a shell with `export`.


## Usage

In a directory with a `docker-compose.yml` file, run the following command to
set the contained environment variables to the current shell:

```shell
export $(dc-env-vars api)
```


## Build

```shell
cargo build --release
```


## Limitations

- environment variables set by `.env` files are not supported
- environment variables used in `docker-compose.yml` which are passed by the
  shell will be included literally

