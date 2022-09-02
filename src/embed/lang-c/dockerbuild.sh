#!/usr/bin/env bash

set -xe

docker run --rm -it buildpack-deps:bullseye \
  /usr/bin/gcc --version
