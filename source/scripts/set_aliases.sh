#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "ERROR: This script must be run via 'source' to ensure aliases are set in the current shell."
  echo
  echo "Usage: source set_aliases.sh"
  return 1 2>/dev/null || exit 1
fi

function gs() {
    git status
}


