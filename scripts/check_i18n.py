#!/usr/bin/env python3
"""Checks interface text keys. Fails when a key used in src is missing from i18n/en.ftl, or when en.ftl holds a key nothing uses.
Prints a coverage line per language file. --prune removes unused keys from en.ftl.
Help center messages are looked up by id (help-art-<slug>-<part>, help-topic-<id>, help-group-<id>, help-ui-ql-<id>), so a key of
those families counts as used when its id appears as a string in src/data/help.rs."""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
KEY = re.compile(r"^([a-z0-9-]+) =", re.M)
def load(p):
    text = p.read_text()
    return {m.group(1): m.start() for m in KEY.finditer(text)}
en = load(ROOT / "i18n/en.ftl")
used = set()
help_data = (ROOT / "src/data/help.rs").read_text()
help_ids = set(re.findall(r'"([a-z0-9-]+)"', help_data))
for f in (ROOT / "src").rglob("*.rs"):
    if f.name != "i18n.rs":
        text = f.read_text()
        used |= set(re.findall(r'i18n::t(?:_args)?\("([^"]+)"', text))
        used |= set(re.findall(r'"([a-z0-9-]+)"', text)) & set(en)  # keys passed around as plain string constants
def family_id(key):
    for prefix in ("help-topic-", "help-group-", "help-ui-ql-"):
        if key.startswith(prefix):
            return key[len(prefix):]
    if key.startswith("help-art-"):
        return key[len("help-art-"):].rsplit("-", 1)[0]
for k in en:
    fid = family_id(k)
    if fid and fid in help_ids:
        used.add(k)
missing = sorted(used - set(en)); unused = sorted(set(en) - used)
if "--prune" in sys.argv and unused:
    p = ROOT / "i18n/en.ftl"
    lines, out, skip = p.read_text().split("\n"), [], False
    for l in lines:
        m = KEY.match(l)
        if m:
            skip = m.group(1) in unused
        elif l and not l[0].isspace():
            skip = False
        if not skip:
            out.append(l)
    p.write_text("\n".join(out))
    unused = []
print(f"keys used {len(used)}, in en.ftl {len(en)}, missing {len(missing)}, unused {len(unused)}")
for k in missing: print("  missing:", k)
for k in unused[:10]: print("  unused:", k)
for f in sorted((ROOT / "i18n").glob("*.ftl")):
    if f.name != "en.ftl":
        d = load(f); print(f"  {f.stem}: {len([k for k in d if k in en])}/{len(en)} translated")
sys.exit(1 if missing or unused else 0)
