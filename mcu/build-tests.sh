#!/bin/bash

set -ex

for dir in `find ./programs -type d -not -path ./programs`; do
    (cd $dir && make -f ../Makefile)
done
