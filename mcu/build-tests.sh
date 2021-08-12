#!/bin/bash

set -ex

cd $(git rev-parse --show-toplevel)

for dir in `find ./cli/programs -type d -not -path ./cli/programs`; do
    (cd $dir && make -f ../Makefile)
done
