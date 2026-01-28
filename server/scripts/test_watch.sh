#!/bin/sh

cargo watch -w ./src -x 'test'
# cargo watch -w ./src -x 'test  all_tests_that_depends_on_db::main -- --nocapture --color always'
