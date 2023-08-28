#!/usr/bin/env bash

VERSION="ghc-9.4.2"
TAGNAME="hcr-devcontainer-lang-haskell:$VERSION"

set -xe

docker build -f ../Dockerfile.build . -t $TAGNAME
