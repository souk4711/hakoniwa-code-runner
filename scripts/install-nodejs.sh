#!/bin/bash

VERSION="18.8.0"
INSTALLDIR="/opt/nodejs-$VERSION"

set -xe

curl -fSsL "https://nodejs.org/dist/v$VERSION/node-v$VERSION-linux-x64.tar.xz" -o /tmp/nodejs-$VERSION.tar.gz
sudo mkdir -p $INSTALLDIR
sudo tar -xf /tmp/nodejs-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/nodejs-$VERSION.tar.gz
