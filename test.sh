#!/bin/sh
set -e

find . -mindepth 2 -name test.sh -type f -perm /a+x \
  -print -not -execdir {} \; -quit
