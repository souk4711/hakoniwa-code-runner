#!/bin/bash

VERSION="3.10.6"
INSTALLDIR="/opt/python-$VERSION"

set -xe

mkdir -p /tmp/python-$VERSION
curl -fSsL "https://www.python.org/ftp/python/$VERSION/Python-$VERSION.tar.xz" -o /tmp/python-$VERSION.tar.gz
tar -xf /tmp/python-$VERSION.tar.gz -C /tmp/python-$VERSION --strip-components=1
rm /tmp/python-$VERSION.tar.gz

mkdir -p $INSTALLDIR
cd /tmp/python-$VERSION
./configure --prefix=$INSTALLDIR
make -j "$(nproc)"
make -j "$(nproc)" install
rm -rf /tmp/python-$VERSION
