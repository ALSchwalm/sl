#!/bin/bash

set -e
set -o xtrace

tempfile=$(mktemp)

sl_command="$@"

# Use screen to avoid sl capturing the screen during tests,
# but use a subshell so we can still get the retcode
screen -mD sh -c "${sl_command}; echo \$? > ${tempfile}"

# Pull the retcode from the screen session into a variable
retcode=$(cat "${tempfile}")

# Do some cleanup
rm "${tempfile}"

# Convert the contents of the file to an actual retcode
exit "${retcode}"
