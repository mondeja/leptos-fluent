#!/bin/bash

EXITCODE=0
BROWSER=$1

set -x
wasm-pack test --headless "--$BROWSER" tests/end2end || EXITCODE=1

for dir in tests/end2end/*/; do
  if [ "$dir" = "tests/end2end/helpers/" ] || [ "$dir" = "tests/end2end/tests/" ]; then
    continue;
  fi
  wasm-pack test --headless "--$BROWSER" "$dir" || EXITCODE=1
done

exit $EXITCODE
