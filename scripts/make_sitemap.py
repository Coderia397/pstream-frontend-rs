#!/usr/bin/env python3
"""Writes public/sitemap.xml from the routes and the help data, so every help article and policy is listed.
Run by the Trunk pre build hook. Reads src/data/help.rs for article slugs and policy paths."""
import re, pathlib, datetime
ROOT = pathlib.Path(__file__).resolve().parent.parent
SITE = "https://streams.watch"
data = (ROOT / "src/data/help.rs").read_text()
slugs = re.findall(r'Article \{ slug: "([a-z0-9-]+)"', data)
docs = re.findall(r'LegalDoc \{ id: "[a-z0-9-]+", path: "([^"]+)"', data)
pages = ["/browse", "/browse/films", "/browse/series", "/latest", "/", "/login", "/signup", "/help", "/help/contact"] + docs
today = datetime.date.today().isoformat()
def url(path, priority):
    return f"  <url><loc>{SITE}{path}</loc><lastmod>{today}</lastmod><priority>{priority}</priority></url>"
lines = ['<?xml version="1.0" encoding="UTF-8"?>', '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">']
lines += [url(p, "0.8" if p in ("/browse", "/help", "/") else "0.6") for p in pages]
lines += [url(f"/help/article/{s}", "0.5") for s in slugs]
lines.append("</urlset>")
out = ROOT / "public/sitemap.xml"
text = "\n".join(lines) + "\n"
# written only when it changed: a write on every build makes `trunk serve` see a changed file and build again, for ever (and reload the phone each time)
if not out.exists() or out.read_text() != text:
    out.write_text(text)
print(f"sitemap: {len(pages)} pages, {len(slugs)} articles")
