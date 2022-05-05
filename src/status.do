#!/usr/bin/env bash
find . -type f -name '*.rs' | xargs redo-ifchange
cargo test --lib &>$3
