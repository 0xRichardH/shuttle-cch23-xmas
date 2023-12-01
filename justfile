#!/usr/bin/env -S just --justfile

local:
  hurl --variable host=http://localhost:8000 --verbose test.hurl

remote:
  hurl --variable host=https://cch23-xmas.shuttleapp.rs --verbose test.hurl
