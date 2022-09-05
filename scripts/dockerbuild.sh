#!/usr/bin/env bash

REPO_ROOT="$( cd -- "$(dirname "$0")/.." >/dev/null 2>&1 || exit; pwd -P )"

for langdir in "$REPO_ROOT"/.devcontainer/lang-* ; do
  langname=$(basename "$langdir")
  langname=${langname#"lang-"}
  if [ "$1" == "all" ] || [ "$1" == "$langname" ]; then
    (set -xe; cd "$langdir"; [ -f ./dockerbuild.sh ]; sh ./dockerbuild.sh)
  fi
done
