#!/usr/bin/env python3
"""Puts the closing brace of a plural message on its own line in every i18n/*.ftl, as Fluent needs it. A model writes
`*[other] { $count } titles}` on one line; the parser rejects that. Run after translate.py:  python3 scripts/i18n/tidy.py"""
import pathlib, re

ROOT = pathlib.Path(__file__).resolve().parents[2]

def tidy(text: str) -> str:
    parts = re.split(r"\n(?=[A-Za-z][A-Za-z0-9_-]*\s*=)", text)
    out = []
    for part in parts:
        lines = part.rstrip("\n").split("\n")
        if "->" in part and lines[-1].strip() != "}" and lines[-1].count("}") > lines[-1].count("{") and lines[-1].rstrip().endswith("}"):
            lines[-1] = lines[-1].rstrip()[:-1].rstrip()
            lines.append("}")
            part = "\n".join(lines) + ("\n" if part.endswith("\n") else "")
        out.append(part)
    return "\n".join(out)

if __name__ == "__main__":
    for f in sorted((ROOT / "i18n").glob("*.ftl")):
        t = f.read_text()
        n = tidy(t)
        if n != t:
            f.write_text(n)
            print("tidied", f.name)
