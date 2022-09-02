#!/usr/bin/env bash

VERSION="1.63.0"
TAGNAME="hcr-rust:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
