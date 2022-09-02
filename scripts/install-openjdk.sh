#!/bin/bash

VERSION="18"
INSTALLDIR="/opt/openjdk-$VERSION"
DOWNLOADURI="https://download.java.net/java/GA/jdk18.0.2.1/db379da656dc47308e138f21b33976fa/1/GPL/openjdk-18.0.2.1_linux-x64_bin.tar.gz"

set -xe

curl -fSsL $DOWNLOADURI -o /tmp/openjdk-$VERSION.tar.gz
sudo mkdir -p $INSTALLDIR
sudo tar -xf /tmp/openjdk-$VERSION.tar.gz -C $INSTALLDIR --strip-components=1
rm /tmp/openjdk-$VERSION.tar.gz
