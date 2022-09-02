#!/bin/bash

VERSION="4.8.2"
INSTALLDIR="/opt/nodejs-18.8.0"

set -xe

sudo $INSTALLDIR/bin/npm install --prefix $INSTALLDIR -g typescript@$VERSION
