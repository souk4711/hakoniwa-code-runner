#!/usr/bin/env bash

VERSION="openjdk-18"
TAGNAME="hcr-java:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
