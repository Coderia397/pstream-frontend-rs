#!/usr/bin/env python3
"""Writes src/data/licences.json: every Rust crate the app is built from (normal dependencies for the wasm target, read with
`cargo metadata`) and every JavaScript package listed under "dependencies" in package.json, with name, version and licence.
The Open source licences screen shows this file. Run it after changing dependencies:  python3 scripts/make_licences.py"""
import json, pathlib, subprocess, sys
ROOT = pathlib.Path(__file__).resolve().parent.parent
meta = json.loads(subprocess.check_output(["cargo", "metadata", "--format-version", "1", "--filter-platform", "wasm32-unknown-unknown", "--manifest-path", str(ROOT / "Cargo.toml")], cwd=ROOT))
by_id = {p["id"]: p for p in meta["packages"]}
root = meta["resolve"]["root"]
nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
seen, stack = set(), [root]
while stack:
    i = stack.pop()
    if i in seen:
        continue
    seen.add(i)
    for d in nodes[i]["deps"]:
        # normal dependencies only: skip build and dev kinds
        if any(k.get("kind") in (None, "normal") for k in d["dep_kinds"]):
            stack.append(d["pkg"])
items = []
for i in seen:
    if i == root:
        continue
    p = by_id[i]
    items.append({"name": p["name"], "version": p["version"], "license": p.get("license") or p.get("license_file") or "see source", "kind": "rust"})
pkg = json.loads((ROOT / "package.json").read_text())
for name in sorted(pkg.get("dependencies", {})):
    f = ROOT / "node_modules" / name / "package.json"
    if f.exists():
        j = json.loads(f.read_text())
        lic = j.get("license") or "see source"
        items.append({"name": name, "version": j.get("version", ""), "license": lic if isinstance(lic, str) else str(lic), "kind": "js"})
# build tools whose output ships with the app but that are not Cargo or npm dependencies
for name, version, lic in (("Tailwind CSS", "4", "MIT"),):
    items.append({"name": name, "version": version, "license": lic, "kind": "tool"})
items.sort(key=lambda x: (x["name"].lower(), x["version"]))
out = ROOT / "src/data/licences.json"
out.write_text(json.dumps(items, indent=0, ensure_ascii=False) + "\n")
print(len(items), "packages written to", out.relative_to(ROOT), file=sys.stderr)
