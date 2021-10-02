#!/bin/sh

# Prerequisites
# rustup toolchain install nightly
# rustup component add llvm-tools-preview
# cargo install rustfilt cargo-binutils

# cargo clean;

RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="target/debug/rust_web_server-%m.profraw" cargo +nightly test --tests;

cargo +nightly profdata -- merge -sparse target/debug/rust_web_server-*.profraw -o target/debug/rust_web_server.profdata;

cargo +nightly cov -- report \
    --use-color  \
    --ignore-filename-regex='/.cargo/registry' \
    --ignore-filename-regex='.*_spec\.rs$' \
    --instr-profile=target/debug/rust_web_server.profdata \
    --object target/debug/deps/rust_web_server-f0967c694a838d0e;

cargo +nightly cov -- show \
    --use-color \
    --ignore-filename-regex='/.cargo/registry' \
    --ignore-filename-regex='.*_spec\.rs$' \
    --instr-profile=target/debug/rust_web_server.profdata \
    --object target/debug/deps/rust_web_server-f0967c694a838d0e \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=./target/debug \
    --format=html;

echo "\n\nAll files can be found at /target/debug/coverage/src/<Filename>.html";
echo "\nExample: $(pwd)/target/debug/coverage/src/main.rs.html\n\n";

# Reference
# https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/source-based-code-coverage.html
