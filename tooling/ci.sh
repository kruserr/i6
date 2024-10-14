#!/usr/bin/env bash
set -Eeuo pipefail

ci () {
  # cargo install --locked cargo-audit cargo-edit cargo-udeps cargo-geiger cargo-crev cargo-deny

  cargo audit
  cargo upgrade --verbose
  cargo update --verbose

  cargo +nightly fmt --all
  cargo clippy --all-targets --all-features -- -Dwarnings
  cargo test

  cargo +nightly udeps --all-targets
}

ci
