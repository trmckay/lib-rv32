#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

for dir in `find ./tests/programs -type d -not -path ./tests/programs`; do
    (cd $dir && make -f ../Makefile)
done
