#!/usr/bin/env bash
set -Eeuo pipefail

ci () {
  cargo +nightly fmt --all
  cargo clippy --all-targets --all-features -- -Dwarnings
  cargo test
}

ci
