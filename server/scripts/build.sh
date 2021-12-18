#!/bin/sh

PKG_NAME="$(grep 'name\s*=\s*"' Cargo.toml | sed -E 's/.*"(.*)"/\1/')"
TARGET=x86_64-unknown-linux-musl

cargo build --release --target=$TARGET
[ ! -d "release" ] && mkdir release
cp ./target/$TARGET/release/$PKG_NAME ./release
chmod 111 ./release/$PKG_NAME
