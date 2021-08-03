#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

for dir in ./tests/programs/*; do
    if [[ -d "$dir" ]]; then
        cd $dir && make -f ../Makefile
    fi
done
