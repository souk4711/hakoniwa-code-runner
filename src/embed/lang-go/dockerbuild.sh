#!/usr/bin/env bash

VERSION="1.19"
TAGNAME="hcr-go:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
