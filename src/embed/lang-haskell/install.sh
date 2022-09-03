#!/usr/bin/env bash

VERSION="9.4.2"
INSTALLDIR="/opt/haskell-$VERSION"

set -xe

curl --proto '=https' --tlsv1.2 -fSsL https://get-ghcup.haskell.org -o /tmp/get-ghcup.sh
BOOTSTRAP_HASKELL_NONINTERACTIVE=1 \
BOOTSTRAP_HASKELL_NO_UPGRADE=1 \
BOOTSTRAP_HASKELL_VERBOSE=1 \
BOOTSTRAP_HASKELL_GHC_VERSION=$VERSION \
GHCUP_INSTALL_BASE_PREFIX=$INSTALLDIR \
  sh /tmp/get-ghcup.sh
rm /tmp/get-ghcup.sh
rm -rf $INSTALLDIR/.ghcup/ghc/$VERSION/share
rm -rf $INSTALLDIR/.ghcup/share
