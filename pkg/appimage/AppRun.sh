#!/bin/bash

APPDIR="$(dirname "$(readlink -f "$0")")"
export solvio__SERVICE__STATIC_CONTENT_DIR="$APPDIR/usr/share/static"
exec "$APPDIR/usr/bin/solvio" "$@"
