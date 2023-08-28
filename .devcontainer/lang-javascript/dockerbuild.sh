#!/usr/bin/env bash

VERSION="18.8.0"
TAGNAME="hcr-devcontainer-lang-nodejs:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
