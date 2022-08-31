#!/bin/bash

VERSION="3.1.2"
INSTALLDIR="/opt/ruby-$VERSION"

set -xe

mkdir -p /tmp/ruby-$VERSION
curl -fSsL "https://cache.ruby-lang.org/pub/ruby/${VERSION%.*}/ruby-$VERSION.tar.gz" -o /tmp/ruby-$VERSION.tar.gz
tar -xf /tmp/ruby-$VERSION.tar.gz -C /tmp/ruby-$VERSION --strip-components=1
rm /tmp/ruby-$VERSION.tar.gz

mkdir -p $INSTALLDIR
cd /tmp/ruby-$VERSION
./configure --prefix=$INSTALLDIR
make -j "$(nproc)"
make -j "$(nproc)" install
rm -rf /tmp/ruby-$VERSION
