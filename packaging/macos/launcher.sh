!/bin/bash
BINARY="$(dirname "$0")/1gh-bin"
if open -a iTerm "$BINARY" 2>/dev/null; then
  exit 0
fi
open -a Terminal "$BINARY"
