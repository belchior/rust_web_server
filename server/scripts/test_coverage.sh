#!/bin/sh

# Prerequisites
# cargo clean;
# rustup toolchain install nightly
# rustup +nightly component add llvm-tools-preview
# cargo install rustfilt cargo-binutils


PKG_NAME="$(grep 'name\s*=\s*"' Cargo.toml | sed -E 's/.*"(.*)"/\1/')"
COVERAGE_DATA="target/debug/coverage"
COVERAGE_PATH="coverage"

mkdir -p "$COVERAGE_DATA"

rm -f "target/debug/deps/$PKG_NAME"*

RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="$COVERAGE_DATA/$PKG_NAME-%m.profraw" cargo +nightly test -- --test-threads=1;
cargo +nightly profdata -- merge -sparse $COVERAGE_DATA/$PKG_NAME-*.profraw -o $COVERAGE_DATA/$PKG_NAME.profdata;

COVERAGE_ID="$(ls target/debug/deps/$PKG_NAME-*.d | head -n1 | cut -d " " -f1 | sed -E 's/.*-(.*)\.d$/\1/')"

cargo +nightly cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc/.*' \
    --ignore-filename-regex='./*_spec\.rs$' \
    --ignore-filename-regex='/cargo/registry' \
    --instr-profile=$COVERAGE_DATA/$PKG_NAME.profdata \
    --object target/debug/deps/$PKG_NAME-$COVERAGE_ID;

cargo +nightly cov -- show \
    --use-color \
    --ignore-filename-regex='/rustc/.*' \
    --ignore-filename-regex='./*_spec\.rs$' \
    --ignore-filename-regex='/cargo/registry' \
    --instr-profile=$COVERAGE_DATA/$PKG_NAME.profdata \
    --object target/debug/deps/$PKG_NAME-$COVERAGE_ID \
    --show-instantiations \
    --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=$COVERAGE_PATH \
    --format=html;

echo "\n\nAll files can be found at:\n$(pwd)/$COVERAGE_PATH/index.html\n\n";

# Reference
# https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html
