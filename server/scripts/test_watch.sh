#!/bin/sh

cargo watch -w ./src -x 'test'
# cargo watch -w ./src -x 'test -- --nocapture --color always'
