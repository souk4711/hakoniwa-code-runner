#!/usr/bin/env bash

VERSION=""
INSTALLDIR="/opt/nodejs-18.8.0"

set -xe

PATH="$INSTALLDIR/bin:$PATH" \
  $INSTALLDIR/bin/tsc --version
