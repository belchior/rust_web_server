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

start server in development mode

```sh
docker-compose up server
```

build server for production

```sh
docker-compose run --rm server ./scripts/build.sh
```

running tests

```sh
# in watch mode
docker-compose run --rm test_server

# single execution
docker-compose run --rm test_server ./scripts/test.sh

# single execution with coverage
docker-compose run --rm test_server ./scripts/test_coverage.sh
```

debug database

```sh
# in development environment
docker-compose exec mongodb mongo learning

# in test environment
docker-compose exec mongodb mongo test_learning
```

**Client**

install dependencies

```sh
npm ci
```

start dev

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
