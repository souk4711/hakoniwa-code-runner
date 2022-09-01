#!/bin/bash

VERSION="2.100.0"
INSTALLDIR="/opt/d-$VERSION"

set -xe

curl -fSsL "http://downloads.dlang.org/releases/2.x/$VERSION/dmd.$VERSION.linux.tar.xz" -o /tmp/d-$VERSION.tar.gz
sudo mkdir -p $INSTALLDIR
sudo tar -xf /tmp/d-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/d-$VERSION.tar.gz
