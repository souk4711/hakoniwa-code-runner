#!/usr/bin/env bash

VERSION="3.1.2"
TAGNAME="hcr-ruby:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
