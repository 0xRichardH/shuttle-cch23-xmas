#!/usr/bin/env -S just --justfile

local day='*':
  hurl --variable host=http://localhost:8000 --test --glob "./test/integration/{{day}}.hurl"

remote:
  hurl --variable host=https://cch23-xmas.shuttleapp.rs --test --glob "./test/integration/*.hurl"

watch day:
  cargo watch -w src -qcs "just local {{day}}" -d 2 

dev:
  cargo watch -w src -qcx "shuttle run" -E RUST_LOG="cch23_xmas=trace,axum::rejection=trace" -E RUST_BACKTRACE=1

validate day:
  cch23-validator {{day}}

validate-all:
  just validate --all

watch-validate day:
  cargo watch -w src -qcs "just validate {{day}}" -d 2

deploy:
  cargo shuttle deploy --allow-dirty
