// Builds the whole brand kit from the Baby Blocks pixel font (Colorful Typhoon, linkware, personal non commercial use:
// http://www.geocities.jp/kitschlabo/). The font file is not in the repo; pass its path.
//   node scripts/make_brand_kit.mjs "/path/to/baby blocks.ttf"
// Writes ./brand/ (the whole kit) and copies the files the website serves into ./assets/logos and ./assets/email.
// Needs opentype.js and @resvg/resvg-js (devDependencies). Recolor everything by editing the COLORS block and running again.
import fs from 'fs';
import path from 'path';
import { execFileSync } from 'child_process';
import opentype from 'opentype.js';
import { Resvg } from '@resvg/resvg-js';

const COLORS = {
  primary: '#2f6bff',   // the brand color: mark, tiles, favicon, buttons in email
  ink: '#ffffff',       // wordmark on dark surfaces
  void: '#0b0b0c',      // dark surface, and the wordmark on light surfaces
  panel: '#16161a',     // raised dark surface
  paper: '#f3f1ec',     // light surface
};
const NAME = 'Streams';
const SITE = 'https://streams.watch';
const SUPPORT = 'support@streams.watch';

const fontPath = process.argv[2];
if (!fontPath) { console.error('usage: node scripts/make_brand_kit.mjs "<font.ttf>"'); process.exit(1); }
const root = path.resolve('brand');
fs.rmSync(root, { recursive: true, force: true });
const dir = (...p) => { const d = path.join(root, ...p); fs.mkdirSync(d, { recursive: true }); return d; };
const put = (p, data) => { fs.mkdirSync(path.dirname(p), { recursive: true }); fs.writeFileSync(p, data); };

// ---------- glyph outlines ----------
const buf = fs.readFileSync(fontPath);
const font = opentype.parse(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength));
const outline = (text) => { const p = font.getPath(text, 0, 0, font.unitsPerEm); const b = p.getBoundingBox(); return { d: p.toPathData(2), x: b.x1, y: b.y1, w: b.x2 - b.x1, h: b.y2 - b.y1 }; };
const word = outline(NAME);
const mark = outline(NAME[0]);
const wordAspect = word.w / word.h;

// ---------- svg builders ----------
const vb = (g) => `${g.x} ${g.y} ${g.w} ${g.h}`;
const pathEl = (g, fill, crisp = true) => `<path fill="${fill}"${crisp ? ' shape-rendering="crispEdges"' : ''} d="${g.d}"/>`;
const wordSvg = (fill, h = 256) => `<svg xmlns="http://www.w3.org/2000/svg" width="${Math.round(h * wordAspect)}" height="${h}" viewBox="${vb(word)}" role="img" aria-label="${NAME}"><title>${NAME}</title>${pathEl(word, fill)}</svg>\n`;
const markSvg = (fill, s = 256) => `<svg xmlns="http://www.w3.org/2000/svg" width="${s}" height="${s}" viewBox="${vb(mark)}" role="img" aria-label="${NAME}"><title>${NAME}</title>${pathEl(mark, fill)}</svg>\n`;
// a nested logo at a position inside a larger canvas
const nested = (g, fill, x, y, w, h) => `<svg x="${x}" y="${y}" width="${w}" height="${h}" viewBox="${vb(g)}">${pathEl(g, fill)}</svg>`;
const markAt = (fill, cx, cy, size) => nested(mark, fill, cx - size / 2, cy - size / 2, size, size);
const wordAt = (fill, cx, cy, width) => { const h = width / wordAspect; return nested(word, fill, cx - width / 2, cy - h / 2, width, h); };
const canvas = (w, h, inner, bg) => `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">${bg ? `<rect width="${w}" height="${h}" fill="${bg}"/>` : ''}${inner}</svg>`;
const tile = (size, scale, radius, bg, fg) => canvas(size, size, `<rect width="${size}" height="${size}" rx="${size * radius}" fill="${bg}"/>${markAt(fg, size / 2, size / 2, size * scale)}`);
const tileFlat = (size, scale, bg, fg) => canvas(size, size, markAt(fg, size / 2, size / 2, size * scale), bg);

// ---------- png ----------
const png = (svg, width, file, background) => {
  const r = new Resvg(svg, { fitTo: { mode: 'width', value: width }, background: background || undefined, shapeRendering: 1 });
  put(file, r.render().asPng());
};
const pngFixed = (svg, w, h, file) => { // for canvases already drawn at their final size
  put(file, new Resvg(svg, { fitTo: { mode: 'width', value: w }, shapeRendering: 1 }).render().asPng());
};
const sq = (size, file, svg) => png(svg, size, file);

// ---------- ico and icns ----------
const ico = (file, sizes, svgFor) => {
  const imgs = sizes.map((s) => Buffer.from(new Resvg(svgFor(s), { fitTo: { mode: 'width', value: s }, shapeRendering: 1 }).render().asPng()));
  const head = Buffer.alloc(6 + 16 * sizes.length);
  head.writeUInt16LE(1, 2); head.writeUInt16LE(sizes.length, 4);
  let off = head.length;
  sizes.forEach((s, i) => { const e = 6 + i * 16; head.writeUInt8(s >= 256 ? 0 : s, e); head.writeUInt8(s >= 256 ? 0 : s, e + 1); head.writeUInt16LE(1, e + 4); head.writeUInt16LE(32, e + 6); head.writeUInt32LE(imgs[i].length, e + 8); head.writeUInt32LE(off, e + 12); off += imgs[i].length; });
  put(file, Buffer.concat([head, ...imgs]));
};

const P = COLORS.primary, I = COLORS.ink, V = COLORS.void, L = COLORS.paper;

// =====================================================================================================================
// 1. LOGOS (svg and png)
// =====================================================================================================================
const L1 = dir('logo');
const variants = { white: I, black: V, primary: P };
for (const [n, c] of Object.entries(variants)) {
  put(path.join(L1, `streams-wordmark-${n}.svg`), wordSvg(c));
  put(path.join(L1, `streams-mark-${n}.svg`), markSvg(c));
  for (const w of [256, 512, 1024, 2048]) png(wordSvg(c), w, path.join(L1, `png/streams-wordmark-${n}-${w}.png`));
  for (const s of [64, 128, 256, 512, 1024]) png(markSvg(c), s, path.join(L1, `png/streams-mark-${n}-${s}.png`));
}
// the wordmark on its own tile colors, for when a file must carry its own background
put(path.join(L1, 'streams-wordmark-on-dark.svg'), canvas(2048, 640, wordAt(I, 1024, 320, 1700), V));
put(path.join(L1, 'streams-wordmark-on-light.svg'), canvas(2048, 640, wordAt(V, 1024, 320, 1700), L));
put(path.join(L1, 'streams-wordmark-on-primary.svg'), canvas(2048, 640, wordAt(I, 1024, 320, 1700), P));
// stacked lockup: mark above the word set small (the word without its own cube is not drawn; the wordmark already holds the cube)
put(path.join(L1, 'streams-lockup-square-dark.svg'), canvas(1024, 1024, wordAt(I, 512, 512, 760), V));
put(path.join(L1, 'streams-lockup-square-light.svg'), canvas(1024, 1024, wordAt(V, 512, 512, 760), L));
// app icon artwork, vector
put(path.join(L1, 'streams-icon-primary.svg'), tile(1024, 0.6, 0.22, P, I));
put(path.join(L1, 'streams-icon-dark.svg'), tile(1024, 0.6, 0.22, V, P));
put(path.join(L1, 'streams-icon-light.svg'), tile(1024, 0.6, 0.22, L, P));
put(path.join(L1, 'streams-icon-square-primary.svg'), tileFlat(1024, 0.6, P, I));
// single color for stamps, embroidery, engraving, fax
put(path.join(L1, 'streams-wordmark-mono-black.svg'), wordSvg('#000000'));
put(path.join(L1, 'streams-mark-mono-black.svg'), markSvg('#000000'));

// =====================================================================================================================
// 2. WEB: favicons, apple, windows, safari, pwa
// =====================================================================================================================
const W = dir('web');
put(path.join(W, 'favicon.svg'), markSvg(P, 64));
const fav = (s) => markSvg(P, s);
for (const s of [16, 32, 48, 64, 96, 128, 192, 256]) png(fav(s), s, path.join(W, `favicon-${s}.png`));
ico(path.join(W, 'favicon.ico'), [16, 32, 48], fav);
// light-on-dark tab bars: a white favicon is not needed, the blue mark reads on both
put(path.join(W, 'safari-pinned-tab.svg'), `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${vb(mark)}"><path d="${mark.d}"/></svg>\n`);
for (const s of [57, 60, 72, 76, 114, 120, 144, 152, 167, 180]) png(tileFlat(s, 0.6, P, I), s, path.join(W, `apple-touch-icon-${s}.png`));
png(tileFlat(180, 0.6, P, I), 180, path.join(W, 'apple-touch-icon.png'));
for (const s of [192, 512]) {
  png(tile(s, 0.6, 0.22, P, I), s, path.join(W, `icon-${s}.png`));
  png(tileFlat(s, 0.5, P, I), s, path.join(W, `icon-maskable-${s}.png`));
  png(tileFlat(s, 0.6, V, P), s, path.join(W, `icon-dark-${s}.png`));
}
// microsoft tiles
for (const [n, w, h] of [['70x70', 128, 128], ['150x150', 270, 270], ['310x310', 558, 558]]) png(tileFlat(w, 0.55, P, I), w, path.join(W, `mstile-${n}.png`));
pngFixed(canvas(558, 270, markAt(I, 279, 135, 150), P), 558, 270, path.join(W, 'mstile-310x150.png'));
put(path.join(W, 'browserconfig.xml'), `<?xml version="1.0" encoding="utf-8"?>\n<browserconfig><msapplication><tile><square70x70logo src="/assets/logos/mstile-70x70.png"/><square150x150logo src="/assets/logos/mstile-150x150.png"/><wide310x150logo src="/assets/logos/mstile-310x150.png"/><square310x310logo src="/assets/logos/mstile-310x310.png"/><TileColor>${P}</TileColor></tile></msapplication></browserconfig>\n`);
put(path.join(W, 'manifest.webmanifest'), JSON.stringify({
  name: NAME, short_name: NAME, description: 'Find your next film or series.', start_url: '/browse', scope: '/', display: 'standalone',
  orientation: 'any', background_color: V, theme_color: V, categories: ['entertainment', 'video'],
  icons: [
    { src: '/assets/logos/icon-192.png', sizes: '192x192', type: 'image/png', purpose: 'any' },
    { src: '/assets/logos/icon-512.png', sizes: '512x512', type: 'image/png', purpose: 'any' },
    { src: '/assets/logos/icon-maskable-192.png', sizes: '192x192', type: 'image/png', purpose: 'maskable' },
    { src: '/assets/logos/icon-maskable-512.png', sizes: '512x512', type: 'image/png', purpose: 'maskable' },
    { src: '/assets/logos/favicon.svg', sizes: 'any', type: 'image/svg+xml', purpose: 'any' },
  ],
  shortcuts: [
    { name: 'Films', url: '/browse/films', icons: [{ src: '/assets/logos/icon-192.png', sizes: '192x192' }] },
    { name: 'Series', url: '/browse/series', icons: [{ src: '/assets/logos/icon-192.png', sizes: '192x192' }] },
    { name: 'My List', url: '/my-list', icons: [{ src: '/assets/logos/icon-192.png', sizes: '192x192' }] },
  ],
}, null, 2) + '\n');
// iOS launch screens (apple-touch-startup-image), dark background with the mark
const splash = [['iphone-15-pro-max', 1290, 2796, 430, 932, 3], ['iphone-15-pro', 1179, 2556, 393, 852, 3], ['iphone-14', 1170, 2532, 390, 844, 3], ['iphone-13-mini', 1080, 2340, 360, 780, 3], ['iphone-11', 828, 1792, 414, 896, 2], ['iphone-se', 750, 1334, 375, 667, 2], ['ipad-pro-12.9', 2048, 2732, 1024, 1366, 2], ['ipad-pro-11', 1668, 2388, 834, 1194, 2], ['ipad-10.9', 1640, 2360, 820, 1180, 2], ['ipad-mini', 1488, 2266, 744, 1133, 2]];
const splashLinks = [];
for (const [n, pw, ph, cw, ch, r] of splash) {
  pngFixed(canvas(pw, ph, markAt(P, pw / 2, ph / 2, Math.min(pw, ph) * 0.26), V), pw, ph, path.join(W, 'splash', `splash-${n}.png`));
  splashLinks.push(`<link rel="apple-touch-startup-image" media="(device-width: ${cw}px) and (device-height: ${ch}px) and (-webkit-device-pixel-ratio: ${r}) and (orientation: portrait)" href="/assets/logos/splash/splash-${n}.png" />`);
}
put(path.join(W, 'head-snippet.html'), `<!-- Paste into <head>. The website already carries the first block; the splash links are optional. -->
<link rel="icon" href="/assets/logos/favicon.svg" type="image/svg+xml" />
<link rel="icon" href="/assets/logos/favicon.ico" sizes="48x48" />
<link rel="icon" type="image/png" sizes="32x32" href="/assets/logos/favicon-32.png" />
<link rel="icon" type="image/png" sizes="16x16" href="/assets/logos/favicon-16.png" />
<link rel="apple-touch-icon" href="/assets/logos/apple-touch-icon.png" />
<link rel="mask-icon" href="/assets/logos/safari-pinned-tab.svg" color="${P}" />
<link rel="manifest" href="/assets/logos/manifest.webmanifest" />
<meta name="msapplication-config" content="/assets/logos/browserconfig.xml" />
<meta name="msapplication-TileColor" content="${P}" />
<meta name="theme-color" content="${V}" />
<meta property="og:image" content="${SITE}/assets/logos/og-image.png" />
<meta name="twitter:card" content="summary_large_image" />
${splashLinks.join('\n')}
`);

// =====================================================================================================================
// 3. DESKTOP APPS: windows ico, macos icns, linux png
// =====================================================================================================================
const D = dir('desktop');
ico(path.join(D, 'icon.ico'), [16, 24, 32, 48, 64, 128, 256], (s) => tile(s, 0.62, 0.18, P, I));
const iconset = dir('desktop', 'icon.iconset');
for (const [s, n] of [[16, 'icon_16x16'], [32, 'icon_16x16@2x'], [32, 'icon_32x32'], [64, 'icon_32x32@2x'], [128, 'icon_128x128'], [256, 'icon_128x128@2x'], [256, 'icon_256x256'], [512, 'icon_256x256@2x'], [512, 'icon_512x512'], [1024, 'icon_512x512@2x']]) png(tile(s, 0.62, 0.2237, P, I), s, path.join(iconset, `${n}.png`));
try { execFileSync('iconutil', ['-c', 'icns', iconset, '-o', path.join(D, 'icon.icns')]); } catch (e) { console.warn('icns skipped (iconutil missing)'); }
for (const s of [16, 24, 32, 48, 64, 128, 256, 512]) png(tile(s, 0.62, 0.18, P, I), s, path.join(D, 'linux', `${s}x${s}.png`));

// =====================================================================================================================
// 4. iOS and ANDROID
// =====================================================================================================================
const IOS = dir('ios', 'AppIcon.appiconset');
const iosSizes = [[20, 1], [20, 2], [20, 3], [29, 1], [29, 2], [29, 3], [40, 1], [40, 2], [40, 3], [60, 2], [60, 3], [76, 1], [76, 2], [83.5, 2], [1024, 1]];
const iosImages = iosSizes.map(([pt, sc]) => ({ pt, sc, px: Math.round(pt * sc), file: `icon-${Math.round(pt * sc)}.png` }));
for (const px of new Set(iosImages.map((i) => i.px))) png(tileFlat(px, 0.6, P, I), px, path.join(IOS, `icon-${px}.png`));
png(tileFlat(1024, 0.6, V, P), 1024, path.join(IOS, 'icon-1024-dark.png'));
png(tileFlat(1024, 0.6, '#000000', '#ffffff'), 1024, path.join(IOS, 'icon-1024-tinted.png'));
put(path.join(IOS, 'Contents.json'), JSON.stringify({
  images: [
    { filename: 'icon-1024.png', idiom: 'universal', platform: 'ios', size: '1024x1024' },
    { appearances: [{ appearance: 'luminosity', value: 'dark' }], filename: 'icon-1024-dark.png', idiom: 'universal', platform: 'ios', size: '1024x1024' },
    { appearances: [{ appearance: 'luminosity', value: 'tinted' }], filename: 'icon-1024-tinted.png', idiom: 'universal', platform: 'ios', size: '1024x1024' },
  ], info: { author: 'streams-brand-kit', version: 1 },
}, null, 2) + '\n');
dir('ios', 'legacy-sizes');
for (const i of iosImages) fs.copyFileSync(path.join(IOS, `icon-${i.px}.png`), path.join(root, 'ios', 'legacy-sizes', `AppIcon-${i.pt}@${i.sc}x.png`));

const A = dir('android');
const dpi = { mdpi: 1, hdpi: 1.5, xhdpi: 2, xxhdpi: 3, xxxhdpi: 4 };
for (const [n, f] of Object.entries(dpi)) {
  const res = path.join(A, 'res', `mipmap-${n}`);
  png(tile(Math.round(48 * f), 0.6, 0.18, P, I), Math.round(48 * f), path.join(res, 'ic_launcher.png'));
  png(canvas(48 * f, 48 * f, `<circle cx="${24 * f}" cy="${24 * f}" r="${24 * f}" fill="${P}"/>${markAt(I, 24 * f, 24 * f, 48 * f * 0.52)}`), Math.round(48 * f), path.join(res, 'ic_launcher_round.png'));
  const l = Math.round(108 * f);
  png(canvas(l, l, markAt(I, l / 2, l / 2, l * 0.42)), l, path.join(res, 'ic_launcher_foreground.png'));
  png(canvas(l, l, markAt('#ffffff', l / 2, l / 2, l * 0.42)), l, path.join(res, 'ic_launcher_monochrome.png'));
}
put(path.join(A, 'res', 'values', 'ic_launcher_background.xml'), `<?xml version="1.0" encoding="utf-8"?>\n<resources><color name="ic_launcher_background">${P}</color></resources>\n`);
const adaptive = `<?xml version="1.0" encoding="utf-8"?>\n<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n  <background android:drawable="@color/ic_launcher_background"/>\n  <foreground android:drawable="@mipmap/ic_launcher_foreground"/>\n  <monochrome android:drawable="@mipmap/ic_launcher_monochrome"/>\n</adaptive-icon>\n`;
put(path.join(A, 'res', 'mipmap-anydpi-v26', 'ic_launcher.xml'), adaptive);
put(path.join(A, 'res', 'mipmap-anydpi-v26', 'ic_launcher_round.xml'), adaptive);
png(tileFlat(512, 0.6, P, I), 512, path.join(A, 'play-store-icon-512.png'));
pngFixed(canvas(1024, 500, wordAt(I, 512, 250, 560), V), 1024, 500, path.join(A, 'play-store-feature-graphic-1024x500.png'));
// android tv
pngFixed(canvas(320, 180, wordAt(I, 160, 90, 190), V), 320, 180, path.join(A, 'tv-banner-320x180.png'));
pngFixed(canvas(640, 360, wordAt(I, 320, 180, 380), V), 640, 360, path.join(A, 'tv-banner-640x360.png'));

// =====================================================================================================================
// 5. TV platforms
// =====================================================================================================================
const T = dir('tv');
pngFixed(canvas(290, 218, wordAt(I, 145, 109, 200), V), 290, 218, path.join(T, 'roku-channel-poster-hd-290x218.png'));
pngFixed(canvas(540, 405, wordAt(I, 270, 202, 380), V), 540, 405, path.join(T, 'roku-channel-poster-fhd-540x405.png'));
pngFixed(canvas(1920, 720, wordAt(I, 960, 360, 900), V), 1920, 720, path.join(T, 'tvos-top-shelf-1920x720.png'));
pngFixed(canvas(1280, 720, wordAt(I, 640, 360, 640), V), 1280, 720, path.join(T, 'fire-tv-icon-1280x720.png'));
pngFixed(canvas(1920, 1080, wordAt(I, 960, 540, 900), V), 1920, 1080, path.join(T, 'tv-splash-1920x1080.png'));

// =====================================================================================================================
// 6. SOCIAL: avatars, banners, share images, posts
// =====================================================================================================================
const S = dir('social');
const avatar = (s, bg, fg) => tileFlat(s, 0.56, bg, fg);
for (const s of [200, 400, 512, 1024]) png(avatar(s, P, I), s, path.join(S, 'avatars', `avatar-primary-${s}.png`));
png(avatar(400, V, P), 400, path.join(S, 'avatars', 'avatar-dark-400.png'));
png(avatar(400, L, P), 400, path.join(S, 'avatars', 'avatar-light-400.png'));
const banner = (w, h, name, wordWidth, bg = V, fg = I) => pngFixed(canvas(w, h, wordAt(fg, w / 2, h / 2, wordWidth), bg), w, h, path.join(S, 'banners', name));
banner(1500, 500, 'x-twitter-header-1500x500.png', 760);
banner(1584, 396, 'linkedin-banner-1584x396.png', 700);
banner(2560, 1440, 'youtube-banner-2560x1440.png', 1000);
banner(820, 312, 'facebook-cover-820x312.png', 420);
banner(960, 540, 'discord-server-banner-960x540.png', 520);
banner(1920, 384, 'reddit-banner-1920x384.png', 700);
banner(1500, 500, 'bluesky-banner-1500x500.png', 760);
banner(1128, 191, 'linkedin-company-cover-1128x191.png', 520);
banner(1280, 640, 'github-social-preview-1280x640.png', 700);
banner(1200, 630, 'open-graph-1200x630.png', 760);
banner(1200, 600, 'twitter-card-1200x600.png', 760);
banner(1600, 900, 'twitch-offline-1920x1080.png'.replace('1920x1080', '1600x900'), 760);
banner(1080, 1080, 'post-square-1080x1080.png', 760);
banner(1080, 1350, 'post-portrait-1080x1350.png', 780);
banner(1080, 1920, 'story-1080x1920.png', 800);
banner(1080, 566, 'post-landscape-1080x566.png', 640);
banner(1200, 630, 'open-graph-primary-1200x630.png', 760, P, I);
banner(1200, 630, 'open-graph-light-1200x630.png', 760, L, V);
banner(1280, 720, 'video-thumbnail-1280x720.png', 700);
banner(2560, 1440, 'zoom-teams-background-2560x1440.png', 900, V, I);
dir('social', 'emoji'); for (const s of [32, 64, 128]) png(tile(s, 0.74, 0.18, P, I), s, path.join(S, 'emoji', `streams-emoji-${s}.png`));
png(tileFlat(512, 0.62, P, I), 512, path.join(S, 'github-org-avatar-500.png'));

// =====================================================================================================================
// 7. EMAIL: logo images (png only, mail clients do not render svg), signatures, templates, bimi, gravatar
// =====================================================================================================================
const E = dir('email');
const ew = [['logo-white', I, V], ['logo-black', V, null], ['logo-primary', P, null]];
for (const [n, fill] of ew) { png(wordSvg(fill), 240, path.join(E, 'images', `${n}.png`)); png(wordSvg(fill), 480, path.join(E, 'images', `${n}@2x.png`)); }
png(markSvg(P), 96, path.join(E, 'images', 'mark-primary.png')); png(markSvg(P), 192, path.join(E, 'images', 'mark-primary@2x.png'));
png(tileFlat(512, 0.6, P, I), 512, path.join(E, 'images', 'avatar-512.png')); // also the Gravatar and Google account image
pngFixed(canvas(1200, 320, wordAt(I, 600, 160, 700), V), 1200, 320, path.join(E, 'images', 'header-dark@2x.png'));
pngFixed(canvas(1200, 320, wordAt(I, 600, 160, 700), P), 1200, 320, path.join(E, 'images', 'header-primary@2x.png'));
// BIMI: SVG Tiny PS, square, solid background, no scripts or raster. Showing it in inboxes needs a published DMARC policy and a mark certificate.
put(path.join(E, 'bimi-logo.svg'), `<?xml version="1.0" encoding="UTF-8"?>\n<svg version="1.2" baseProfile="tiny-ps" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><title>${NAME}</title><rect width="512" height="512" fill="${P}"/><svg x="108" y="108" width="296" height="296" viewBox="${vb(mark)}"><path fill="${I}" d="${mark.d}"/></svg></svg>\n`);

const base = ({ preheader, heading, body, button, link, note }) => `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="light dark"><meta name="supported-color-schemes" content="light dark"><title>${NAME}</title></head>
<body style="margin:0;padding:0;background:${L};">
<div style="display:none;max-height:0;overflow:hidden;opacity:0;color:transparent;">${preheader}</div>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="background:${L};"><tr><td align="center" style="padding:32px 16px;">
<table role="presentation" width="560" cellpadding="0" cellspacing="0" style="width:100%;max-width:560px;background:#ffffff;border-radius:14px;overflow:hidden;">
<tr><td align="center" style="background:${V};padding:30px 24px;"><a href="${SITE}"><img src="${SITE}/assets/email/logo-white@2x.png" width="150" alt="${NAME}" style="display:block;border:0;height:auto;"></a></td></tr>
<tr><td style="padding:36px 36px 8px;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;color:${V};">
<h1 style="margin:0 0 14px;font-size:24px;line-height:1.25;font-weight:700;">${heading}</h1>
<p style="margin:0 0 24px;font-size:16px;line-height:1.55;color:#3a3a3d;">${body}</p>
${button ? `<table role="presentation" cellpadding="0" cellspacing="0"><tr><td style="background:${P};border-radius:8px;"><a href="${link}" style="display:inline-block;padding:14px 28px;font-size:16px;font-weight:600;color:#ffffff;text-decoration:none;">${button}</a></td></tr></table>
<p style="margin:22px 0 0;font-size:13px;line-height:1.5;color:#6b6b70;">If the button does not work, copy this address into your browser:<br><a href="${link}" style="color:${P};word-break:break-all;">${link}</a></p>` : ''}
${note ? `<p style="margin:22px 0 0;font-size:13px;line-height:1.5;color:#6b6b70;">${note}</p>` : ''}
</td></tr>
<tr><td style="padding:28px 36px 34px;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;font-size:12px;line-height:1.5;color:#8a8a8f;border-top:1px solid #ececec;margin-top:24px;">
Sent by ${NAME}. Questions? Write to <a href="mailto:${SUPPORT}" style="color:#8a8a8f;">${SUPPORT}</a>.<br>You received this email because of an action on ${SITE.replace('https://', '')}.</td></tr>
</table></td></tr></table></body></html>
`;
const tpl = path.join(E, 'templates');
const mails = {
  'supabase-confirm-signup': { preheader: 'Confirm your email to start.', heading: 'Confirm your email', body: 'Welcome. Confirm your email address to finish creating your account.', button: 'Confirm email', link: '{{ .ConfirmationURL }}', note: 'If you did not sign up, you can ignore this email.' },
  'supabase-magic-link': { preheader: 'Your sign in link.', heading: 'Your sign in link', body: 'Use the button to sign in. The link works once and expires soon.', button: 'Sign in', link: '{{ .ConfirmationURL }}', note: 'If you did not ask for this, you can ignore this email.' },
  'supabase-reset-password': { preheader: 'Reset your password.', heading: 'Reset your password', body: 'We received a request to reset the password for {{ .Email }}.', button: 'Choose a new password', link: '{{ .ConfirmationURL }}', note: 'If you did not ask for this, ignore this email and your password stays the same.' },
  'supabase-invite': { preheader: 'You have been invited.', heading: 'You have been invited', body: 'Accept the invite to create your account.', button: 'Accept invite', link: '{{ .ConfirmationURL }}', note: '' },
  'supabase-change-email': { preheader: 'Confirm your new email address.', heading: 'Confirm your new email', body: 'Confirm the change of your email address from {{ .Email }} to {{ .NewEmail }}.', button: 'Confirm change', link: '{{ .ConfirmationURL }}', note: 'If you did not ask for this, contact support right away.' },
  'supabase-reauthentication': { preheader: 'Your confirmation code.', heading: 'Confirm it is you', body: 'Enter this code to continue: <strong style="font-size:22px;letter-spacing:4px;">{{ .Token }}</strong>', button: '', link: '', note: 'The code expires soon. If you did not ask for it, ignore this email.' },
  'contact-received': { preheader: 'We got your message.', heading: 'We got your message', body: 'Thanks for writing. We read every message and reply as soon as we can.', button: '', link: '', note: '' },
  'welcome': { preheader: 'Welcome.', heading: 'Welcome to Streams', body: 'Your account is ready. Pick a profile and find your next film or series.', button: 'Open Streams', link: SITE + '/browse', note: '' },
};
for (const [n, m] of Object.entries(mails)) put(path.join(tpl, `${n}.html`), base(m));
put(path.join(E, 'signature.html'), `<table cellpadding="0" cellspacing="0" style="font-family:-apple-system,Helvetica,Arial,sans-serif;font-size:13px;color:#3a3a3d;"><tr><td style="padding-right:14px;"><img src="${SITE}/assets/email/mark-primary@2x.png" width="48" height="48" alt="${NAME}" style="display:block;border:0;"></td><td style="border-left:2px solid ${P};padding-left:14px;"><strong style="font-size:14px;color:${V};">Your name</strong><br>Role, ${NAME}<br><a href="${SITE}" style="color:${P};text-decoration:none;">${SITE.replace('https://', '')}</a> &middot; <a href="mailto:${SUPPORT}" style="color:#3a3a3d;text-decoration:none;">${SUPPORT}</a></td></tr></table>\n`);

// =====================================================================================================================
// 8. ANIMATED: loader and intro (SVG with built in animation, no script)
// =====================================================================================================================
const AN = dir('animated');
const px = 1024 / 16; // one font pixel is 64 units
const dots = [0, 1, 2, 3, 4].map((i) => `<rect x="${1300 + i * 220}" y="-64" width="128" height="128" fill="${P}"><animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.25;0.7;1" dur="1.8s" begin="${i * 0.25}s" repeatCount="indefinite"/></rect>`).join('');
put(path.join(AN, 'loader-streaming-pixels.svg'), `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-64 -800 2600 1100" width="520" height="220" role="img" aria-label="Loading"><title>Loading</title><path fill="${I}" shape-rendering="crispEdges" d="${mark.d}"/>${dots}</svg>\n`);
put(path.join(AN, 'loader-mark-pulse.svg'), `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${vb(mark)}" width="128" height="128" role="img" aria-label="Loading"><title>Loading</title><path fill="${P}" shape-rendering="crispEdges" d="${mark.d}"><animate attributeName="opacity" values="1;0.35;1" dur="1.4s" repeatCount="indefinite"/></path></svg>\n`);
const steps = [0, 1, 2, 3, 4, 5, 6, 7].map((n) => Math.round(word.w * Math.min(1, (n + 1) / 8)));
put(path.join(AN, 'intro-wordmark-reveal.svg'), `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${vb(word)}" width="740" height="204" role="img" aria-label="${NAME}"><title>${NAME}</title><defs><clipPath id="r"><rect x="${word.x}" y="${word.y}" width="${word.w}" height="${word.h}"><animate attributeName="width" values="0;${steps.join(';')}" calcMode="discrete" dur="1.1s" fill="freeze"/></rect></clipPath></defs><path fill="${I}" shape-rendering="crispEdges" clip-path="url(#r)" d="${word.d}"/></svg>\n`);

// =====================================================================================================================
// 9. OTHER: browser extension, print, watermark, app store screenshots frame, document headers
// =====================================================================================================================
const O = dir('other');
for (const s of [16, 32, 48, 128]) png(tile(s, 0.7, 0.18, P, I), s, path.join(O, 'browser-extension', `icon-${s}.png`));
pngFixed(canvas(440, 280, wordAt(I, 220, 140, 280), V), 440, 280, path.join(O, 'browser-extension', 'store-small-tile-440x280.png'));
pngFixed(canvas(1400, 560, wordAt(I, 700, 280, 700), V), 1400, 560, path.join(O, 'browser-extension', 'store-marquee-1400x560.png'));
// image watermark: wordmark at 60 percent white, transparent png
png(wordSvg('rgba(255,255,255,0.6)'), 600, path.join(O, 'watermark-white-60-600.png'));
png(wordSvg('rgba(0,0,0,0.6)'), 600, path.join(O, 'watermark-black-60-600.png'));
// document header (letterhead strip), 2480 wide for A4 at 300 dpi
pngFixed(canvas(2480, 300, wordAt(V, 360, 150, 480), L), 2480, 300, path.join(O, 'letterhead-header-a4-2480x300.png'));
// slides master cover
pngFixed(canvas(1920, 1080, wordAt(I, 960, 540, 760), V), 1920, 1080, path.join(O, 'slides-cover-1920x1080.png'));
// website placeholders
pngFixed(canvas(1600, 900, markAt(P, 800, 450, 300), V), 1600, 900, path.join(O, 'image-placeholder-1600x900.png'));
pngFixed(canvas(1600, 900, wordAt(I, 800, 450, 640), V), 1600, 900, path.join(O, 'maintenance-1600x900.png'));
pngFixed(canvas(1600, 900, wordAt(I, 800, 450, 640), V), 1600, 900, path.join(O, 'error-404-background-1600x900.png'));

// =====================================================================================================================
// 10. WEB COPIES (what the site serves) and docs
// =====================================================================================================================
const web = path.resolve('assets/logos');
fs.mkdirSync(web, { recursive: true });
const copy = (from, to) => { fs.mkdirSync(path.dirname(to), { recursive: true }); fs.copyFileSync(from, to); };
for (const f of fs.readdirSync(W)) { const p = path.join(W, f); if (fs.statSync(p).isFile() && f !== 'head-snippet.html') copy(p, path.join(web, f)); }
for (const f of fs.readdirSync(path.join(W, 'splash'))) copy(path.join(W, 'splash', f), path.join(web, 'splash', f));
copy(path.join(L1, 'streams-wordmark-white.svg'), path.join(web, 'streams-wordmark-white.svg'));
copy(path.join(L1, 'streams-wordmark-black.svg'), path.join(web, 'streams-wordmark-black.svg'));
copy(path.join(L1, 'streams-wordmark-primary.svg'), path.join(web, 'streams-wordmark-primary.svg'));
copy(path.join(L1, 'streams-mark-white.svg'), path.join(web, 'streams-mark-white.svg'));
copy(path.join(L1, 'streams-mark-primary.svg'), path.join(web, 'streams-mark-primary.svg'));
copy(path.join(S, 'banners', 'open-graph-1200x630.png'), path.join(web, 'og-image.png'));
const webEmail = path.resolve('assets/email');
for (const f of fs.readdirSync(path.join(E, 'images'))) copy(path.join(E, 'images', f), path.join(webEmail, f));

// App Store and Play Store reject icons that carry an alpha channel, so flatten those to plain RGB
try {
  const flat = [path.join(root, 'ios', 'AppIcon.appiconset'), path.join(root, 'ios', 'legacy-sizes'), path.join(root, 'android')];
  execFileSync('python3', ['-c', `import sys,os\nfrom PIL import Image\nfor d in sys.argv[1:]:\n  for r,_,fs in os.walk(d):\n    for f in fs:\n      p=os.path.join(r,f)\n      if f.endswith('.png') and ('foreground' not in f and 'monochrome' not in f and 'splash' not in f and 'tv-banner' not in f and 'feature' not in f and 'round' not in f and f!='ic_launcher.png'):\n        Image.open(p).convert('RGB').save(p)`, ...flat]);
} catch (e) { console.warn('alpha flatten skipped (python3 with Pillow needed):', e.message); }

put(path.join(root, 'colors.json'), JSON.stringify(COLORS, null, 2) + '\n');
put(path.join(root, 'README.md'), `# ${NAME} brand kit

Generated by \`node scripts/make_brand_kit.mjs "<path to baby blocks.ttf>"\`. Edit the COLORS block at the top of that script and run it again to recolor everything.

The logo is set in the Baby Blocks pixel font by Colorful Typhoon (http://www.geocities.jp/kitschlabo/), linkware for non commercial personal use. The capital S becomes the pixel cube, the rest is plain pixel type. See assets/logos/CREDITS.md.

## Colors
| Name | Value | Use |
|---|---|---|
| Primary | ${P} | mark, tiles, favicon, email buttons |
| Ink | ${I} | wordmark on dark |
| Void | ${V} | dark surface, wordmark on light |
| Panel | ${COLORS.panel} | raised dark surface |
| Paper | ${L} | light surface |

## Rules
* The wordmark is white on dark, black on light, or primary on dark. No other colors, no outlines, no shadows, no stretching.
* Clear space around the logo: one pixel cube height (the height of the S block) on every side.
* Minimum size: wordmark 80 px wide on screen (20 mm in print), mark alone 16 px.
* The mark alone (the cube S) is for icons, avatars and favicons; the wordmark is for headers, banners and email.
* The pixels are a 16 by 16 grid; at 16, 32 and 64 px the mark is exact and sharp, other sizes may soften.

## What is where
| Folder | Contents |
|---|---|
| logo/ | wordmark and mark as SVG in white, black, primary, mono; on dark, light and primary backgrounds; square lockups; PNG at 256 to 2048 |
| web/ | favicon (svg, ico, png 16 to 256), apple touch icons, maskable and dark icons, Windows tiles and browserconfig, Safari pinned tab, manifest with shortcuts, 10 iOS launch screens, head snippet |
| desktop/ | Windows icon.ico, macOS icon.icns and iconset, Linux png sizes |
| ios/ | AppIcon.appiconset with light, dark and tinted 1024, every legacy size, Contents.json |
| android/ | launcher icons for every density, round, adaptive foreground and monochrome layers, adaptive XML, Play Store icon and feature graphic, Android TV banners |
| tv/ | Roku posters, Apple TV top shelf, Fire TV icon, TV splash |
| social/ | avatars, 20 banners and share images (X, LinkedIn, YouTube, Facebook, Discord, Reddit, Bluesky, GitHub, Open Graph, Twitter card, stories and posts), emoji, GitHub org avatar |
| email/ | PNG logos at 1x and 2x, headers, mark, avatar (also Gravatar), BIMI logo, signature, branded Supabase auth templates, contact receipt and welcome templates |
| animated/ | loader with streaming pixels, pulsing mark, wordmark reveal (SVG, no script) |
| other/ | browser extension icons and store tiles, watermarks, A4 letterhead header, slides cover, placeholders |

## Notes
* Email clients do not render SVG, so email uses the PNG files, which the site serves from /assets/email/.
* BIMI needs a published DMARC policy of quarantine or reject and a mark certificate before inboxes show it; the logo file is ready.
* The Supabase templates use Supabase's template variables ({{ .ConfirmationURL }}, {{ .Email }}, {{ .NewEmail }}, {{ .Token }}); paste each file into Authentication, Email Templates.
* The icon sets assume the primary color; iOS tinted and dark variants are included.
`);
const count = (d) => fs.readdirSync(d, { recursive: true }).filter((f) => fs.statSync(path.join(d, f)).isFile()).length;
console.log('brand kit files:', count(root), '| wordmark aspect', wordAspect.toFixed(3));
