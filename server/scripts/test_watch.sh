#!/bin/sh

# TODO find a way to run tests in parellel:
# https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively
cargo watch -w ./src -x 'test -- --test-threads=1'
# To print logs
# cargo watch -w ./src -x 'test -- --test-threads=1 --nocapture --color always'
