#!/bin/bash

EXITCODE=0

cd end2end
wasm-pack test --headless --$1
if [ $? -ne 0 ]; then
  EXITCODE=1
fi

for dir in */; do
  if [ "$dir" = "tests-helpers/" ] || [ "$dir" = "tests/" ]; then
    continue;
  fi
  cd $dir
  wasm-pack test --headless --$1
  if [ $? -ne 0 ]; then
    EXITCODE=1
  fi
  cd ..
done

cd ..

exit $EXITCODE
