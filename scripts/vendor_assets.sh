#!/bin/sh
# Copies the web fonts we serve from node_modules (pinned in package.json) into assets/phosphor and fonts/inter.
# Run after changing the Phosphor or Inter version; the copies are committed so a build does not need this step.
set -eu
cd "$(dirname "$0")/.."
for w in regular bold fill; do
  d=assets/phosphor/$w; mkdir -p "$d"
  src=node_modules/@phosphor-icons/web/src/$w
  name=$(ls "$src"/*.woff2 | head -1 | xargs basename)
  cp "$src/$name" "$d/$name"
  # keep only the woff2 source; every browser we support reads it
  awk -v n="$name" '/^@font-face/{inface=1} inface&&/src:/{print "  src: url(\"./" n "\") format(\"woff2\");"; skip=1; next} skip&&/;/{skip=0; next} skip{next} {print} /^}/{inface=0}' "$src/style.css" > "$d/style.css"
done
mkdir -p fonts/inter
cp node_modules/@fontsource-variable/inter/files/inter-latin-wght-normal.woff2 fonts/inter/
cp node_modules/@fontsource-variable/inter/files/inter-latin-ext-wght-normal.woff2 fonts/inter/
