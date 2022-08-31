#!/bin/bash

VERSION="1.19"
INSTALLDIR="/opt/go-$VERSION"

set -xe

mkdir $INSTALLDIR
curl -fSsL "https://go.dev/dl/go$VERSION.linux-amd64.tar.gz" -o /tmp/go-$VERSION.tar.gz
tar -xf /tmp/go-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/go-$VERSION.tar.gz
