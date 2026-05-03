#! /bin/bash
set -o errexit -o nounset -o pipefail
cd "$(dirname "$0")"

run() {
  echo 'exec> cargo' "$@" >&2
  cargo "$@"
}

run clippy -- -D warnings
run test
RUSTFLAGS='-D warnings' run doc
