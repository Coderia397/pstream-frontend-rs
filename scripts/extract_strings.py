#!/usr/bin/env python3
"""Finds interface text in the Rust views and writes a draft locales/en.ftl plus a key map. Dry run by default; --apply rewrites the
source to call crate::i18n::t("key"). Only clean cases are touched: a string literal alone on a line inside a view, and
title, placeholder, aria-label, alt and label attributes with a plain string value."""
import re, sys, pathlib, json, collections
ROOT = pathlib.Path(__file__).resolve().parent.parent
SKIP = {"avatars.rs", "languages.rs", "genres.rs", "page_genres.rs", "legal.rs", "brand.rs", "keys.rs", "i18n.rs"}
RE_TEXT = re.compile(r'(?m)^(\s*)"([A-Z\u00c0-\u00de][^"\\{}]*[A-Za-z0-9.!?…)])"\s*$')
RE_ATTR = re.compile(r'\b(title|placeholder|aria-label|alt|label)="([A-Z][^"{}\\]{2,})"')
def slug(text, n=5):
    w = re.sub(r"[^a-z0-9 ]", "", text.lower()).split()[:n]
    return "-".join(w) or "text"
keys = {}; used = collections.Counter(); out = []
def key_for(path, text):
    base = f"{path.stem.replace('_', '-')}-{slug(text)}"
    if text in keys: return keys[text]
    k = base; i = 2
    while k in used and used[k] != text: k = f"{base}-{i}"; i += 1
    used[k] = text; keys[text] = k; return k
apply = "--apply" in sys.argv; n_text = n_attr = 0
for f in sorted((ROOT / "src").rglob("*.rs")):
    if f.name in SKIP: continue
    s = f.read_text(); o = s
    def t(m):
        global n_text; n_text += 1
        return f'{m.group(1)}{{crate::i18n::t("{key_for(f, m.group(2))}")}}'
    def a(m):
        global n_attr; n_attr += 1
        return f'{m.group(1)}=crate::i18n::t("{key_for(f, m.group(2))}")'
    s = RE_TEXT.sub(t, s); s = RE_ATTR.sub(a, s)
    if apply and s != o: f.write_text(s)
ftl = "\n".join(f"{k} = {v}" for v, k in sorted(keys.items(), key=lambda x: x[1]))
(ROOT / "locales").mkdir(exist_ok=True)
(pathlib.Path(sys.argv[sys.argv.index("--out") + 1]) if "--out" in sys.argv else ROOT / "locales/en.draft.ftl").write_text(ftl + "\n")
print({"text": n_text, "attr": n_attr, "keys": len(keys), "applied": apply})
