#!/usr/bin/env python3
"""Checks interface text keys. Fails when a key used in src is missing from i18n/en.ftl, or when en.ftl holds a key nothing uses.
Prints a coverage line per language file. --prune removes unused keys from en.ftl."""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
def load(p):
    return dict(l.split(" = ", 1) for l in p.read_text().splitlines() if re.match(r"^[a-z0-9-]+ = ", l))
en = load(ROOT / "i18n/en.ftl")
used = set()
for f in (ROOT / "src").rglob("*.rs"):
    if f.name != "i18n.rs": used |= set(re.findall(r'i18n::t(?:_args)?\("([^"]+)"', f.read_text()))
missing = sorted(used - set(en)); unused = sorted(set(en) - used)
if "--prune" in sys.argv and unused:
    p = ROOT / "i18n/en.ftl"
    p.write_text("\n".join(l for l in p.read_text().splitlines() if not (re.match(r"^[a-z0-9-]+ = ", l) and l.split(" = ")[0] in unused)) + "\n")
    unused = []
print(f"keys used {len(used)}, in en.ftl {len(en)}, missing {len(missing)}, unused {len(unused)}")
for k in missing: print("  missing:", k)
for k in unused[:10]: print("  unused:", k)
for f in sorted((ROOT / "i18n").glob("*.ftl")):
    if f.name != "en.ftl":
        d = load(f); print(f"  {f.stem}: {len([k for k in d if k in en])}/{len(en)} translated")
sys.exit(1 if missing or unused else 0)
