#!/usr/bin/env bash

VERSION="3.1.3"
TAGNAME="hcr-scala:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
