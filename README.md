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

install dependencies

```shell
cargo install --path .
```

start dev

```shell
scripts/start_dev.sh
```

test

```shell
scripts/test.sh
```

build

```shell
scripts/build.sh
```

**Client**

install dependencies

```shell
npm ci
```

start dev

```shell
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
