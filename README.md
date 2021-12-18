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

**Server**

The `base_image` contains all compiled dependencies to be used in `development` `testing` and build to `production`.

All commands depends on `base_image` to run, to build it manually you should run the command below

```sh
docker build -f Dockerfile.base -t base_image .
```

To start's server in development mode

```sh
docker-compose up server
```

To build server for production

```sh
# before run this command you must build the base_image first
docker build -t rust_web_server .
```

To run tests

```sh
# in watch mode
docker-compose run --rm test_server

# for single execution
docker-compose run --rm test_server ./scripts/test.sh

# for single execution with coverage
docker-compose run --rm test_server ./scripts/test_coverage.sh
```

To debug database

```sh
# in development environment
docker-compose exec mongodb mongo learning

# in test environment
docker-compose exec mongodb mongo test_learning
```

**Client**

To install dependencies

```sh
npm ci
```

To start's in development mode

```sh
npm start
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

