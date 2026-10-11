#!/bin/sh
# Copies the product name from src/brand.rs into index.html (the page title). Runs in the Trunk pre_build hook.
# index.html is rewritten only when the title changes, so `trunk serve` does not rebuild in a loop.
set -eu
cd "$(dirname "$0")/.."
NAME=$(sed -n 's/^pub const NAME: &str = "\(.*\)";/\1/p' src/brand.rs)
[ -n "$NAME" ] || { echo "apply_brand: NAME not found in src/brand.rs" >&2; exit 1; }
sed "s#<title>.*</title>#<title>$NAME</title>#" index.html > index.html.tmp
if cmp -s index.html index.html.tmp; then rm index.html.tmp; else mv index.html.tmp index.html; fi
