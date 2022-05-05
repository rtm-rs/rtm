#!/usr/bin/env bash
set -e

SELF=$(basename "${0##*/}" .do)
find . -type f -name '*.do' -print0 | \
    xargs -0 echo | \
    sed -e 's/\.do//g' -e "s/\.\/$SELF//g" | \
    xargs redo-ifchange

# Find conflicts between here and our remote.
# Do the merge in memory - don't touch the index, nor the working tree.
git fetch origin main
if git merge-tree "$(git merge-base FETCH_HEAD main)" main FETCH_HEAD | grep -e '<<<<<<<' -e '>>>>>>>'
then
  echo "Merge conflict. May need manual intervention.  Exiting."
  exit 1
else
  # Looks OK.  Pull changes in for real.
  git pull --set-upstream origin main
fi
git push --set-upstream origin main
# git subrepo push rtm-attributes
# git subrepo push rtm-cli
# git subrepo push rtm-csv
# git subrepo push rtm-devtools
# git subrepo push rtm-examples
# git subrepo push rtm-json
# git subrepo push rtm-macros
# git subrepo push rtm-schema
# git subrepo push rtm-sql
# git subrepo push rtm-tests
# git subrepo push rtm-yaml
