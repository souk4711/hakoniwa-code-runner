#!/usr/bin/env bash

# VERSION=""
TAGNAME="hcr-nodejs:18.8.0"

set -xe

docker run -e PATH="/opt/nodejs-18.8.0/bin:/usr/bin:/bin" --rm -it $TAGNAME \
  /opt/nodejs-18.8.0/bin/tsc --version
