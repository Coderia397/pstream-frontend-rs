#!/bin/sh
# Fails if brand values are hardcoded outside style/brand.css and src/brand.rs.
cd "$(dirname "$0")/.."
fail=0
chk() { n=$(eval "$2" | wc -l | tr -d ' '); echo "$1: $n"; [ "$n" -eq 0 ] || { eval "$2" | head -5; fail=1; }; }
chk "brand red hex in src"            "grep -rniE '#(e50914|f40612|b80710|b81d24)' src"
chk "surface hex in src"              "grep -rniE '#(141414|181818|1e1e1e|262626|2f2f2f|2a2a2a|3a3a3a)\\b' src"
chk "rgba brand red"                  "grep -rnE 'rgba?\\( ?229, ?9, ?20' src style/input.css"
chk "stock red classes in src"        "grep -rnE '(bg|text|border|ring)-red-[0-9]+' src"
chk "radius with raw px in src"       "grep -rnE 'rounded(-[a-z]+)?-\\[[0-9]+px\\]' src"
chk "raw z-index in src"              "grep -rnE '(^|[^a-z-])z-\\[[0-9]+\\]' src"
chk "raw font size in src"            "grep -rnE 'text-\\[[0-9.]+px\\]' src"
chk "raw shadow in src"               "grep -rnE '(drop-)?shadow-\\[' src"
chk "raw nav heights in src"          "grep -rnE '(h|top)-\\[(80|88)px\\]' src"
chk "raw white or black class"        "grep -rnE '(^|[^a-z-])(bg|text|border|from|to|via|ring|divide|fill|stroke)-(white|black)([^a-z-]|$)' src"
chk "raw duration class"              "grep -rnE '(^|[^a-z-])duration-[0-9]' src"
chk "CDN scripts in index.html"       "grep -nE 'unpkg|jsdelivr' index.html"
chk "window.pstream_ globals in src"  "grep -rn 'js_namespace = window' src"
chk "name literal in src"             "grep -rni 'pstream' src | grep -viE '^src/(brand|keys)\\.rs|pstream_|pstream-|pstream_frontend_rs|pstream\\.watch'"
chk "support email literal in src"    "grep -rn 'support@' src | grep -v '^src/brand.rs'"
exit $fail
