#!/usr/bin/env bash

VERSION="2.100.0"
TAGNAME="hcr-d:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
