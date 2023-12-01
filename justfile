#!/usr/bin/env -S just --justfile

local:
  hurl --variable host=http://localhost:8000 --test --glob "./test/integration/*.hurl"

remote:
  hurl --variable host=https://cch23-xmas.shuttleapp.rs --test --glob "./test/integration/*.hurl"
