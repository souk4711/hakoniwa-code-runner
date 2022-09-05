#!/usr/bin/env bash

VERSION="25.0.4"
TAGNAME="hcr-erlang:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
