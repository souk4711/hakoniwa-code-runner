#!/usr/bin/env bash

VERSION="1.19"
INSTALLDIR="/opt/go-$VERSION"

set -xe

curl -fSsL "https://go.dev/dl/go$VERSION.linux-amd64.tar.gz" -o /tmp/go-$VERSION.tar.gz
sudo mkdir -p $INSTALLDIR
sudo tar -xf /tmp/go-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/go-$VERSION.tar.gz
