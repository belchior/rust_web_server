# Rust Web Server

## Stack

**Server**

- Rust
- Actix Web
- PostgreSQL

**Client**

- TypeScript
- React
- Material UI


## Development

To start developing you should build the `base_image` for `server` and `client` running the command below

```sh
docker-compose build server_base_image client_base_image
```

After that you can start both using the command

```sh
docker-compose run --rm database_seed
docker-compose up server client
```


**Server**

The `base_image` contains all compiled dependencies to be used in `development`, `testing` and build for `production`.

You can build it manually with

```sh
docker-compose build server_base_image
```

To start's server in development mode

```sh
docker-compose run --rm database_seed
docker-compose up server
```

To build server for production

```sh
docker-compose build server_build
```

To run tests

```sh
# in watch mode
docker-compose run --rm server_test

# for single execution
docker-compose run --rm server_test ./scripts/test.sh

# for single execution with coverage
docker-compose run --rm server_test ./scripts/test_coverage.sh
```

To debug database

```sh
# in development environment
docker-compose exec database psql -U postgres -d database

# in test environment
docker-compose exec database psql -U postgres -d test_database
```

**Client**

As the same as server, the `base_image` contains all dependencies to be used in `development`, `testing` and build for `production`.

You can build it manually with

```sh
docker-compose build client_base_image
```

To start's in development mode

```sh
docker-compose up client
```

To build client for production

```sh
docker-compose build client_build
```

## References

Actix Web

- https://actix.rs/docs/getting-started/
- https://docs.rs/actix-web/4.0.1/actix_web/index.html
- https://docs.rs/serde/1.0.136/serde/index.html

PostgreSQL

- https://docs.rs/tokio-postgres/latest/tokio_postgres/
- https://docs.rs/deadpool-postgres/latest/deadpool_postgres/

Testing Rust Code

- https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
- https://doc.rust-lang.org/book/ch11-02-running-tests.html#controlling-how-tests-are-run
- https://cloudmaker.dev/actix-integration-tests/
- https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
- https://www.infinyon.com/blog/2021/04/rust-custom-test-harness/

Generate Code Coverage

- https://doc.rust-lang.org/stable/rustc/instrument-coverage.html
