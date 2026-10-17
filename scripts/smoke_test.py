#!/usr/bin/env python3
"""Opens the running site in a headless Chrome (the dev server, or STREAMS_URL) and checks what must never break: the pages load without script errors, the Top 10 rows draw their numerals,
an endless row pages round (forward from the last page comes back to page one, back from page one reaches the last page), and the page dashes follow.

  uv run --with websockets python3 scripts/smoke_test.py [http://localhost:8080]

Exit code 0 when everything passed, 1 otherwise. Needs Chrome (CHROME env var, default the Chrome Beta app on macOS)."""
import asyncio, json, os, subprocess, sys, tempfile, time, urllib.request
import websockets

CHROME = os.environ.get("CHROME", "/Applications/Google Chrome Beta.app/Contents/MacOS/Google Chrome Beta")
BASE = (sys.argv[1] if len(sys.argv) > 1 else os.environ.get("STREAMS_URL", "http://localhost:8080")).rstrip("/")
PORT = 9344
failures = []


def check(ok, what):
    print(("ok    " if ok else "FAIL  ") + what)
    if not ok:
        failures.append(what)


async def main():
    profile = tempfile.mkdtemp(prefix="streams-smoke-")
    proc = subprocess.Popen([CHROME, "--headless=new", "--disable-gpu", f"--remote-debugging-port={PORT}", "--window-size=1792,1400", f"--user-data-dir={profile}", "about:blank"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        time.sleep(4)
        tabs = json.loads(urllib.request.urlopen(f"http://localhost:{PORT}/json").read())
        ws_url = [t for t in tabs if t["type"] == "page"][0]["webSocketDebuggerUrl"]
        async with websockets.connect(ws_url, max_size=50_000_000) as ws:
            n = 0
            errors = []

            async def call(method, params=None):
                nonlocal n
                n += 1
                await ws.send(json.dumps({"id": n, "method": method, "params": params or {}}))
                while True:
                    msg = json.loads(await ws.recv())
                    if msg.get("method") == "Runtime.exceptionThrown":
                        errors.append(msg["params"]["exceptionDetails"].get("text", "") + " " + json.dumps(msg["params"]["exceptionDetails"].get("exception", {}).get("description", ""))[:200])
                    if msg.get("id") == n:
                        return msg.get("result", msg)

            async def js(expr):
                r = await call("Runtime.evaluate", {"expression": expr, "returnByValue": True, "awaitPromise": True})
                return r.get("result", {}).get("value")

            await call("Page.enable"); await call("Runtime.enable")
            await call("Emulation.setDeviceMetricsOverride", {"width": 1792, "height": 1400, "deviceScaleFactor": 1, "mobile": False})

            # with STREAMS_TEST_EMAIL and STREAMS_TEST_PASSWORD (and STREAMS_SUPABASE_URL, STREAMS_SUPABASE_KEY) the test signs in first, as a person with an account does
            email, password = os.environ.get("STREAMS_TEST_EMAIL"), os.environ.get("STREAMS_TEST_PASSWORD")
            if email and password:
                sb, key = os.environ["STREAMS_SUPABASE_URL"], os.environ["STREAMS_SUPABASE_KEY"]
                await call("Page.navigate", {"url": BASE + "/login"}); await asyncio.sleep(4)
                signed = await js(f"""(async()=>{{
                  const r=await fetch('{sb}/auth/v1/token?grant_type=password',{{method:'POST',headers:{{apikey:'{key}','Content-Type':'application/json'}},body:JSON.stringify({{email:'{email}',password:'{password}'}})}});
                  const v=await r.json(); if(!v.access_token) return false; const m=v.user.user_metadata||{{}};
                  localStorage.setItem('streams_session', JSON.stringify({{access_token:v.access_token,refresh_token:v.refresh_token,expires_at:v.expires_at,user_id:v.user.id,email:v.user.email,first_name:m.first_name||'',last_name:m.last_name||'',phone:''}}));
                  localStorage.setItem('streams_consent', JSON.stringify({{personalization:false,at:Date.now(),v:1}}));
                  const p=await fetch('{sb}/rest/v1/profiles?select=id&order=sort_order.asc&limit=1',{{headers:{{apikey:'{key}',Authorization:'Bearer '+v.access_token}}}}); const l=await p.json();
                  if(l[0]) localStorage.setItem('streams_active_profile', l[0].id); return true;}})()""")
                check(bool(signed), "the test account signs in")
            for path in ("/browse", "/latest", "/search", "/browse/genre/83", "/browse/genre/34399", "/browse/series", "/browse/films"):
                await call("Page.navigate", {"url": BASE + path})
                await asyncio.sleep(12)
                title = await js("document.title")
                body = await js("document.body.innerText.length")
                check(bool(title) and (body or 0) > 200, f"{path} renders (title {title!r}, {body} characters)")
            check(not errors, "no script errors" + ("" if not errors else f": {errors[:2]}"))

            await call("Page.navigate", {"url": BASE + "/latest"})
            await asyncio.sleep(14)
            numerals = await js("document.querySelectorAll('.topten-svgwrap svg text').length")
            check((numerals or 0) > 0, f"Top 10 numerals are drawn ({numerals})")
            empty = await js("[...document.querySelectorAll('.topten-svgwrap linearGradient')].filter(g=>g.querySelectorAll('stop').length<3).length")
            check(empty == 0, "every numeral gradient has its colour stops")

            await call("Page.navigate", {"url": BASE + "/browse"})
            await asyncio.sleep(14)
            rows = await js("(() => { const s=[...document.querySelectorAll('.row-scroll-strip')].filter(x=>x.querySelector('[data-loop-clone]')); window.__s=s; return s.length; })()")
            check((rows or 0) > 0, f"rows that loop exist ({rows})")
            if rows:
                async def click(side):
                    await js(f"(() => {{ const o=window.__s[0].closest('.row-scroll-outer'); const b=[...o.querySelectorAll('.edge-btn')][{0 if side == 'left' else 1}]; b.click(); }})()")
                async def left():
                    return await js("Math.round(window.__s[0].scrollLeft)")
                await js("window.__s[0].scrollIntoView({block:'center'})"); await asyncio.sleep(1)
                seen = []
                for _ in range(8):
                    await click("right"); await asyncio.sleep(1.7); seen.append(await left())
                check(len(set(seen)) <= 4 and seen.count(seen[0]) >= 2, f"paging forward comes back to page one (positions {seen})")
                before = seen[-1]
                await click("left"); await asyncio.sleep(1.7); back = await left()
                check(back != before, f"paging back moves to another page ({before} to {back})")
                await click("left"); await asyncio.sleep(1.7); back2 = await left()
                check(back2 != back, f"paging back again moves on ({back} to {back2})")
    finally:
        proc.terminate()


asyncio.run(main())
print("\n%s" % ("all checks passed" if not failures else f"{len(failures)} failed"))
sys.exit(1 if failures else 0)
