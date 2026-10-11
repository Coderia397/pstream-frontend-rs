#!/usr/bin/env python3
"""Drafts i18n/<lang>.ftl from i18n/en.ftl with a model on Cloudflare Workers AI, then checks every message keeps its
placeholders, tags, links, line markers and plural form. Messages that fail are asked again with the problem named.
  python3 scripts/i18n/translate.py de                 # one language, resumes from scripts/i18n/.cache/de.json
  python3 scripts/i18n/translate.py de --only help-ui  # only keys starting with a prefix
  python3 scripts/i18n/translate.py --list
Needs CF_EMAIL and CF_KEY in the environment (a Cloudflare account that has Workers AI)."""
import json, os, pathlib, re, sys, time, urllib.request, urllib.error, concurrent.futures as cf
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import ftl

ROOT = pathlib.Path(__file__).resolve().parents[2]
CACHE = pathlib.Path(__file__).resolve().parent / ".cache"
ACCOUNT = os.environ.get("CF_ACCOUNT", "524d2ad843ea7c320ef07308197ad7a0")
MODEL = os.environ.get("TRANSLATE_MODEL", "@cf/qwen/qwen3.8-27b")

# language: (English name, plural categories, register, dialect note)
LANGS = {
    "ar": ("Arabic", "zero, one, two, few, many, other", "polite, plain Modern Standard Arabic", "Modern Standard Arabic understood everywhere; right to left text; use Western digits 0-9"),
    "de": ("German", "one, other", "informal 'du' is not used on streaming sites: use the polite but friendly 'Sie'-free imperative form where possible, otherwise 'du'", "Standard German (Germany); noun genders must agree with articles and adjectives"),
    "es": ("Spanish", "one, other", "friendly 'tú'", "neutral Latin American Spanish; noun genders must agree with articles and adjectives"),
    "fr": ("French", "one, other", "friendly 'vous'", "Standard French; noun genders must agree; use épicène wording instead of gendered endings where possible"),
    "hi": ("Hindi", "one, other", "polite 'आप'", "Standard Hindi in Devanagari; verbs agree in gender, so prefer forms that do not show the reader's gender"),
    "it": ("Italian", "one, other", "friendly 'tu' as used by streaming services", "Standard Italian; noun genders must agree"),
    "ja": ("Japanese", "other", "polite です・ます form", "Standard Japanese; no spaces between words"),
    "ko": ("Korean", "other", "polite 해요체/합쇼체 as used by apps (합니다 for system messages)", "Standard Korean"),
    "nl": ("Dutch", "one, other", "friendly 'je'", "Standard Dutch (Netherlands)"),
    "pl": ("Polish", "one, few, many, other", "polite but plain 'Ty' in lower case as apps use", "Standard Polish; noun genders and cases must agree; avoid gendered past tense for the reader"),
    "pt": ("Portuguese", "one, many, other", "friendly 'você'", "Brazilian Portuguese; noun genders must agree"),
    "ru": ("Russian", "one, few, many, other", "polite 'вы'", "Standard Russian; noun genders and cases must agree; avoid gendered past tense for the reader"),
    "sv": ("Swedish", "one, other", "friendly 'du'", "Standard Swedish"),
    "tr": ("Turkish", "one, other", "polite 'siz'", "Standard Turkish; vowel harmony in suffixes after placeholders must still read correctly"),
    "zh": ("Chinese (Simplified)", "other", "polite 您 only where needed, otherwise plain 你", "Simplified Chinese for mainland readers; no spaces between Chinese and numbers"),
}

NEVER = "Streams, Netflix, TMDB, Supabase, Cloudflare, Baby Blocks, Colorful Typhoon, Inter, Phosphor, WhatsApp, Instagram, Snapchat, Chrome, Edge, Firefox, Safari, Brave, Opera, GitHub, DMCA, PIN, VPN, DNS, HLS, WebAssembly, JavaScript, Wi-Fi, Bluetooth, HD, 4K"


def call(messages, max_tokens=6000):
    body = json.dumps({"messages": messages, "max_tokens": max_tokens, "temperature": 0.2, "chat_template_kwargs": {"enable_thinking": False}}).encode()
    req = urllib.request.Request(
        f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT}/ai/run/{MODEL}", data=body,
        headers={"X-Auth-Email": os.environ["CF_EMAIL"], "X-Auth-Key": os.environ["CF_KEY"], "Content-Type": "application/json"})
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=240) as r:
                data = json.load(r)
            res = data.get("result") or {}
            if "response" in res and res["response"] is not None:
                return res["response"] if isinstance(res["response"], str) else json.dumps(res["response"])
            ch = res.get("choices")
            if ch:
                return ch[0]["message"]["content"]
            raise RuntimeError(str(data)[:300])
        except (urllib.error.URLError, TimeoutError, RuntimeError) as e:
            time.sleep(3 * (attempt + 1))
            err = e
    raise RuntimeError(f"model call failed: {err}")


def system_prompt(lang, glossary):
    name, cats, register, dialect = LANGS[lang]
    gl = "\n".join(f'  "{en}" -> "{tr}"' for en, tr in glossary) or "  (none)"
    return f"""You are a professional UI translator working on a streaming website called Streams. Translate the English interface text you are given into {name} ({lang}).
Reply with one JSON object only (no code fences, no comments) that maps each key to its translation.

Rules that the software depends on:
1. Placeholders such as {{ $brand }} or {{ $n }} must stay exactly as written, with the same names. You may move them where {name} needs them.
2. Tags <b>…</b>, <a=/path>…</a>, <a href="…">…</a> stay, and every link address stays unchanged. Translate only the words between tags.
3. Lines that start with a marker keep that marker exactly: "## ", "!! ", ">> ", "@@ tool" (never translate the tool word), "1. 2. 3." numbering, and "- ". Keep blank lines and the number of lines.
4. Plural messages use this syntax, with exactly these categories for {name}: {cats}. Put *[other] as the default and last variant. Example shape:
{{ $n ->
    [one] ...
   *[other] ...
}}
5. These names are never translated: {NEVER}. Keyboard key names (Space, Esc, Ctrl, Cmd, Enter, Tab, M, F, K, J, L) stay as written.

How to write:
- Meaning first: plain, friendly, short wording a person uses in an app, not word for word. Register: {register}.
- Do not guess the reader's gender. Where {name} marks gender on verbs, adjectives or "you", use a neutral form (imperative, infinitive, plural, or rewording). Articles and adjectives must agree with the noun they describe.
- Language variety: {dialect}.
- For common app words (sign in, sign out, account, settings, profile, My List, search, play, resume, subtitles, episode, season, trending, cookies) use the wording that the big streaming apps use in {name}, so people recognise it.
- Keep one consistent word for the same thing: profile, list ("My List"), Kids profile, sign in, account, help center, cookies, subtitles, episode, season.
- Approved translations of some terms (use these exact words when the term appears):
{gl}
- Do not add or remove information. Do not explain. Output JSON only."""


def prompt_for(lang, batch, glossary, problems=None):
    items = [{"key": k, "en": v} for k, v in batch]
    user = json.dumps(items, ensure_ascii=False)
    if problems:
        user += "\n\nYour previous answer had problems. Fix exactly these and answer again for the same keys:\n" + "\n".join(f"- {k}: {p}" for k, p in problems)
    return [{"role": "system", "content": system_prompt(lang, glossary)}, {"role": "user", "content": user}]


def parse(text):
    text = text.strip()
    text = re.sub(r"^```(?:json)?\s*|\s*```$", "", text)
    a, b = text.find("{"), text.rfind("}")
    return json.loads(text[a : b + 1])


def batches(items, limit=2600):
    cur, size = [], 0
    for k, v in items:
        if cur and size + len(v) > limit:
            yield cur
            cur, size = [], 0
        cur.append((k, v))
        size += len(v) + 40
    if cur:
        yield cur


def run(lang, only=None):
    en = ftl.read((ROOT / "i18n/en.ftl").read_text())
    en_map = dict(en)
    CACHE.mkdir(exist_ok=True)
    cache_file = CACHE / f"{lang}.json"
    cache = json.loads(cache_file.read_text()) if cache_file.exists() else {}
    # translations that were made by hand earlier are kept and used as the glossary
    old_file = ROOT / f"i18n/{lang}.ftl"
    hand = {}
    if old_file.exists() and not cache_file.exists():
        for k, v in ftl.read(old_file.read_text()):
            if k in en_map and "\n" not in v and "{" not in en_map[k] and len(en_map[k]) < 45:
                hand[k] = v
    glossary = [(en_map[k], v) for k, v in hand.items()][:60]
    for k, v in hand.items():
        cache.setdefault(k, v)
    todo = [(k, v) for k, v in en if k not in cache and (not only or k.startswith(only))]
    print(f"{lang}: {len(en)} messages, {len(cache)} done, {len(todo)} to translate", flush=True)

    def work(batch):
        got, problems = {}, None
        for attempt in range(3):
            try:
                answer = parse(call(prompt_for(lang, batch, glossary, problems)))
            except Exception as e:  # bad JSON or a failed call: ask again
                problems = [(batch[0][0], f"answer was not valid JSON ({str(e)[:60]})")]
                continue
            problems = []
            for k, v in batch:
                t = answer.get(k)
                if not isinstance(t, str):
                    problems.append((k, "missing from the answer"))
                    continue
                t = t.replace("\r", "").strip("\n")
                bad = ftl.check(v, t)
                if "->" in v and not bad:
                    wanted = set(c.strip() for c in LANGS[lang][1].split(","))
                    have = set(re.findall(r"\[(\w+)\]", t)) | {"other"}
                    if have != wanted:
                        bad = [f"plural categories must be exactly: {', '.join(sorted(wanted))} (found: {', '.join(sorted(have))})"]
                if bad:
                    problems.append((k, "; ".join(bad)))
                else:
                    got[k] = t
            batch = [(k, v) for k, v in batch if k not in got]
            if not batch:
                break
        return got, [k for k, _ in batch]

    failed = []
    with cf.ThreadPoolExecutor(6) as pool:
        for got, bad in pool.map(work, list(batches(todo))):
            cache.update(got)
            failed += bad
            cache_file.write_text(json.dumps(cache, ensure_ascii=False, indent=1))
    print(f"{lang}: translated, {len(failed)} could not be made valid: {failed[:8]}", flush=True)
    return failed


def write_file(lang):
    en = ftl.read((ROOT / "i18n/en.ftl").read_text())
    cache = json.loads((CACHE / f"{lang}.json").read_text())
    out = [(k, cache[k]) for k, _ in en if k in cache]
    (ROOT / f"i18n/{lang}.ftl").write_text(ftl.write(out))
    print(f"{lang}: wrote i18n/{lang}.ftl with {len(out)} of {len(en)} messages")


if __name__ == "__main__":
    args = sys.argv[1:]
    if "--list" in args:
        print(", ".join(LANGS))
        sys.exit(0)
    only = args[args.index("--only") + 1] if "--only" in args else None
    if "--plurals" in args:
        # forget the plural messages of the chosen languages so they are asked again with the exact categories
        en_sel = [k for k, v in ftl.read((ROOT / "i18n/en.ftl").read_text()) if "->" in v]
        for lang in [a for a in args if a in LANGS] or list(LANGS):
            f = CACHE / f"{lang}.json"
            if f.exists():
                c = json.loads(f.read_text())
                for k in en_sel:
                    c.pop(k, None)
                f.write_text(json.dumps(c, ensure_ascii=False, indent=1))
    langs = [a for a in args if a in LANGS] or list(LANGS)
    for lang in langs:
        run(lang, only)
        write_file(lang)
