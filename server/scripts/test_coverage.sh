#!/bin/sh

# Prerequisites
# cargo clean;
# cargo install rustfilt cargo-binutils
# rustup toolchain install $nightly
# rustup +nightly component add llvm-tools-preview

PKG_NAME="$(grep 'name\s*=\s*"' Cargo.toml | sed -E 's/.*"(.*)"/\1/')"
COVERAGE_OUTPUT="coverage"
COVERAGE_DATA="target/debug/coverage"

mkdir -p "$COVERAGE_OUTPUT"
mkdir -p "$COVERAGE_DATA"
rm -f "target/debug/deps/$PKG_NAME"*

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$COVERAGE_DATA/$PKG_NAME-%m.profraw" cargo +nightly test -- --test-threads=1;
cargo +nightly profdata -- merge -sparse $COVERAGE_DATA/$PKG_NAME-*.profraw -o $COVERAGE_DATA/$PKG_NAME.profdata;

COVERAGE_ID="$(ls target/debug/deps/$PKG_NAME-*.d | head -n1 | cut -d " " -f1 | sed -E 's/.*-(.*)\.d$/\1/')"

cargo +nightly cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/cargo/registry' \
    --ignore-filename-regex='./*/mock' \
    --ignore-filename-regex='./*_spec.rs$' \
    --ignore-filename-regex='./main.rs' \
    --instr-profile=$COVERAGE_DATA/$PKG_NAME.profdata \
    --object target/debug/deps/$PKG_NAME-$COVERAGE_ID;

cargo +nightly cov -- show \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/cargo/registry' \
    --ignore-filename-regex='./*/mock' \
    --ignore-filename-regex='./*_spec.rs$' \
    --ignore-filename-regex='./main.rs' \
    --instr-profile=$COVERAGE_DATA/$PKG_NAME.profdata \
    --object target/debug/deps/$PKG_NAME-$COVERAGE_ID \
    --show-instantiations \
    --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=$COVERAGE_OUTPUT \
    --format=html;

echo "\n\nAll files can be found at:\n$(pwd)/$COVERAGE_OUTPUT/index.html\n\n";

# Reference
# https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html
