#!/bin/bash

VERSION="1.63.0"
INSTALLDIR="/opt/rust-$VERSION"

set -xe

curl --proto '=https' --tlsv1.2 -fSsL https://sh.rustup.rs -o /tmp/rustup-init.sh
sudo \
  HOME=$INSTALLDIR \
    sh /tmp/rustup-init.sh -v -y --no-modify-path --default-toolchain=$VERSION
rm /tmp/rustup-init.sh
