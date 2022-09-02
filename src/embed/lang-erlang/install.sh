#!/usr/bin/env bash

VERSION="25.0.4"
INSTALLDIR="/opt/erlang-$VERSION"

set -xe

curl -fSsL "https://github.com/erlang/otp/releases/download/OTP-$VERSION/otp_src_$VERSION.tar.gz" -o /tmp/erlang-$VERSION.tar.gz
mkdir -p /tmp/erlang-$VERSION && tar -xf /tmp/erlang-$VERSION.tar.gz -C /tmp/erlang-$VERSION --strip-components=1
rm /tmp/erlang-$VERSION.tar.gz

cd /tmp/erlang-$VERSION
./configure --prefix=$INSTALLDIR && make -j "$(nproc)"
sudo mkdir -p $INSTALLDIR
sudo make -j "$(nproc)" install
sudo rm -rf /tmp/erlang-$VERSION
