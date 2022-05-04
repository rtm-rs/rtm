#!/usr/bin/env bash
redo-ifchange Cargo src/src tests/tests examples/examples benches/benches
git push --set-upstream origin main
git subrepo push rtm-attributes
git subrepo push rtm-cli
git subrepo push rtm-csv
git subrepo push rtm-devtools
git subrepo push rtm-examples
git subrepo push rtm-json
git subrepo push rtm-macros
git subrepo push rtm-schema
git subrepo push rtm-sql
git subrepo push rtm-tests
git subrepo push rtm-yaml
