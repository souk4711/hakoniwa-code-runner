#!/usr/bin/env bash

VERSION="3.1.3"
INSTALLDIR="/opt/scala-$VERSION"

set -xe

curl -fSsL "https://github.com/lampepfl/dotty/releases/download/$VERSION/scala3-$VERSION.tar.gz" -o /tmp/scala-$VERSION.tar.gz
sudo mkdir -p $INSTALLDIR
sudo tar -xf /tmp/scala-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/scala-$VERSION.tar.gz
