#!/usr/bin/env bash

VERSION="3.10.6"
TAGNAME="hcr-example-multi-python:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
