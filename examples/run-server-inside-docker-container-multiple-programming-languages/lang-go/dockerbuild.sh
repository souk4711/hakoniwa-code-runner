#!/usr/bin/env bash

VERSION="1.19"
TAGNAME="hcr-example-multi-lang-go:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
