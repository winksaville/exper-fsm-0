#!/usr/bin/env bash
set -euo pipefail
# -x for debug
#set -x

REPORT_DIR=./reports/criterion
NOW=$(TZ=UTC date +%y-%m-%dT%H-%M-%S.%3NZ)
HASH=$(git log -1 --pretty=%h)

mkdir -p $REPORT_DIR/$HASH/
cargo criterion --message-format json --history-id $HASH > $REPORT_DIR/$HASH/$NOW.json

# Copy the reports (for now this is "too much")
#cp -ar ./target/criterion/reports $REPORT_DIR/$HASH/$NOW