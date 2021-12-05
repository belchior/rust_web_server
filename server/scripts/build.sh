#!/bin/sh

PKG_NAME="$(grep 'name\s*=\s*"' Cargo.toml | sed -E 's/.*"(.*)"/\1/')"

cargo build --release
cp ./target/release/$PKG_NAME ./release
chmod 111 ./release/$PKG_NAME
