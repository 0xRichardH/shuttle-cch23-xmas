#!/usr/bin/env -S just --justfile

local day='*':
  hurl --variable host=http://localhost:8000 --test --glob "./test/integration/{{day}}.hurl"

remote:
  hurl --variable host=https://cch23-xmas.shuttleapp.rs --test --glob "./test/integration/*.hurl"

watch day:
  cargo watch -qcs "just local {{day}}" -d 3 

dev:
  cargo watch -qcx "shuttle run" -E RUST_LOG="cch23_xmas=trace"

deploy:
  cargo shuttle deploy --allow-dirty
