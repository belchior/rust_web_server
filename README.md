# Rust Web Server

## Stack

**Server**

- Rust
- Actix Web
- MongoDB

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
docker-compose up client server database_seed
```


**Server**

The `base_image` contains all compiled dependencies to be used in `development`, `testing` and build for `production`.

You can build it manually with

```sh
docker-compose build server_base_image
```

To start's server in development mode

```sh
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
docker-compose exec database mongo database

# in test environment
docker-compose exec database mongo test_database
```

**Client**

As the same as server the `base_image` contains all dependencies to be used in `development`, `testing` and build for `production`.

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
- https://docs.rs/actix-web/3.3.2/actix_web/
- https://docs.rs/serde/1.0.126/serde/

MongoDB

- https://developer.mongodb.com/quickstart/rust-crud-tutorial/
- https://developer.mongodb.com/article/serde-improvements/
- https://docs.rs/bson/0.14.1/bson/index.html
- https://docs.rs/mongodb/1.2.1/mongodb
- https://docs.rs/bson/1.2.2/bson

Rust

- https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html
- https://doc.rust-lang.org/book/ch11-02-running-tests.html#controlling-how-tests-are-run
- https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html

