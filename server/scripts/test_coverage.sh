#!/bin/sh

# Prerequisites
# rustup toolchain install nightly
# rustup component add llvm-tools-preview
# cargo install rustfilt cargo-binutils

# cargo clean;

source .env
rm target/debug/$RUST_PKG_NAME-*.profraw
RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="target/debug/$RUST_PKG_NAME-%m.profraw" cargo +nightly test --tests;

cargo +nightly profdata -- merge -sparse target/debug/$RUST_PKG_NAME-*.profraw -o target/debug/$RUST_PKG_NAME.profdata;

cargo +nightly cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc/.*' \
    --ignore-filename-regex='./*_spec\.rs$' \
    --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=target/debug/$RUST_PKG_NAME.profdata \
    --object target/debug/deps/$RUST_PKG_NAME-$RUST_COVERAGE_NUMBER;

cargo +nightly cov -- show \
    --use-color \
    --ignore-filename-regex='/rustc/.*' \
    --ignore-filename-regex='./*_spec\.rs$' \
    --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=target/debug/$RUST_PKG_NAME.profdata \
    --object target/debug/deps/$RUST_PKG_NAME-$RUST_COVERAGE_NUMBER \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=./target/debug \
    --format=html;

echo "\n\nAll files can be found at /target/debug/coverage/src/<Filename>.html";
echo "\nExample: $(pwd)/target/debug/coverage/src/main.rs.html\n\n";

# Reference
# https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/source-based-code-coverage.html
