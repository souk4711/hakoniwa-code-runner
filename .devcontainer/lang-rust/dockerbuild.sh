#!/usr/bin/env bash

VERSION="1.63.0"
TAGNAME="hcr-devcontainer-lang-rust:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
