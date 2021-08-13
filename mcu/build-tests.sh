#!/bin/bash

set -ex

cd $(git rev-parse --show-toplevel)/mcu

for dir in `find ./programs -type d -not -path ./programs`; do
    (cd $dir && make -f ../Makefile)
done
