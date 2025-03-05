#!/bin/bash

EXITCODE=0
BROWSER=$1

set -x
cd tests/end2end
wasm-pack test --headless --$BROWSER
if [ $? -ne 0 ]; then
  EXITCODE=1
fi

for dir in */; do
  if [ "$dir" = "tests-helpers/" ] || [ "$dir" = "tests/" ]; then
    continue;
  fi
  cd $dir
  wasm-pack test --headless --$BROWSER
  if [ $? -ne 0 ]; then
    EXITCODE=1
  fi
  cd ..
done

cd ..
set +x

exit $EXITCODE
