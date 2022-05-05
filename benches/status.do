#!/usr/bin/env bash
find . -name '*.rs' | xargs redo-ifchange
cargo test --benches &>$3
