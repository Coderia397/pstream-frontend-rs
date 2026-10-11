// Builds the animated logo files from the Baby Blocks pixel font: pixels flow in from the left and settle into the wordmark.
//   node scripts/make_animations.mjs "/path/to/baby blocks.ttf"
// Writes brand/animated/ (svg, gif, apng, webp) and brand/email/images/header-animated.gif. Needs opentype.js, @resvg/resvg-js and python3 with Pillow.
import fs from 'fs';
import os from 'os';
import path from 'path';
import { execFileSync } from 'child_process';
import opentype from 'opentype.js';
import { Resvg } from '@resvg/resvg-js';

const P = '#2f6bff', I = '#ffffff', V = '#0b0b0c';
const fontPath = process.argv[2];
if (!fontPath) { console.error('usage: node scripts/make_animations.mjs "<font.ttf>"'); process.exit(1); }
const out = path.resolve('brand/animated');
fs.mkdirSync(out, { recursive: true });
fs.mkdirSync(path.resolve('brand/email/images'), { recursive: true });

const buf = fs.readFileSync(fontPath);
const font = opentype.parse(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength));
const gp = font.getPath('Streams', 0, 0, font.unitsPerEm);
const bb = gp.getBoundingBox();
const word = { d: gp.toPathData(2), x: bb.x1, y: bb.y1, w: bb.x2 - bb.x1, h: bb.y2 - bb.y1 };
const U = 64;                              // one font pixel in font units
const cols = Math.round(word.w / U), rows = Math.round(word.h / U);

// 1. find which grid cells are lit by rasterising the wordmark at 4 px per cell
const SS = 4;
const r = new Resvg(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="${word.x} ${word.y} ${word.w} ${word.h}" width="${cols * SS}" height="${rows * SS}"><path fill="#000" shape-rendering="crispEdges" d="${word.d}"/></svg>`, { shapeRendering: 1 }).render();
const px = r.pixels, W = r.width;
const cells = [];
for (let cy = 0; cy < rows; cy++) for (let cx = 0; cx < cols; cx++) {
  const i = ((cy * SS + SS / 2) * W + (cx * SS + SS / 2)) * 4;
  if (px[i + 3] > 127) cells.push({ cx, cy });
}
console.log('grid', cols, 'x', rows, 'lit cells', cells.length);

// 2. timing per cell: left to right wave with jitter
let seed = 11; const rnd = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
for (const c of cells) { c.delay = (c.cx / (cols - 1)) * 0.95 + rnd() * 0.22; c.dx = (6 + Math.floor(rnd() * 10)) * U; c.dur = 0.42 + rnd() * 0.12; }
const rect = (c, x = 0, o = 1, fill = I) => `<rect x="${word.x + c.cx * U + x}" y="${word.y + c.cy * U}" width="${U}" height="${U}" fill="${fill}"${o < 1 ? ` opacity="${o.toFixed(3)}"` : ''}/>`;

// 3. animated SVG (CSS keyframes, no script): once with freeze, and a looping version
const svgHead = (extra = '') => `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${word.x - 128} ${word.y - 128} ${word.w + 256} ${word.h + 256}" width="1000" height="${Math.round(1000 * (word.h + 256) / (word.w + 256))}" role="img" aria-label="Streams"><title>Streams</title><style>${extra}</style><g shape-rendering="crispEdges">`;
const onceCss = '@keyframes fly{0%{transform:translateX(var(--dx));opacity:0}55%{opacity:1}100%{transform:translateX(0);opacity:1}}rect{animation:fly var(--t) cubic-bezier(.16,.84,.3,1) var(--d) both}@media (prefers-reduced-motion:reduce){rect{animation:none}}';
const loopCss = '@keyframes fly{0%{transform:translateX(var(--dx));opacity:0}14%{transform:translateX(0);opacity:1}78%{transform:translateX(0);opacity:1}92%{transform:translateX(0);opacity:0}100%{transform:translateX(var(--dx));opacity:0}}rect{animation:fly 3.6s cubic-bezier(.16,.84,.3,1) var(--d) infinite}@media (prefers-reduced-motion:reduce){rect{animation:none;opacity:1}}';
const cellSvg = (fill) => cells.map((c) => `<rect x="${word.x + c.cx * U}" y="${word.y + c.cy * U}" width="${U}" height="${U}" fill="${fill}" style="--dx:${-c.dx}px;--d:${c.delay.toFixed(2)}s;--t:${c.dur.toFixed(2)}s"/>`).join('');
fs.writeFileSync(path.join(out, 'intro-pixel-stream.svg'), svgHead(onceCss) + cellSvg(I) + '</g></svg>\n');
fs.writeFileSync(path.join(out, 'intro-pixel-stream-primary.svg'), svgHead(onceCss) + cellSvg(P) + '</g></svg>\n');
fs.writeFileSync(path.join(out, 'loop-pixel-stream.svg'), svgHead(loopCss) + cellSvg(I) + '</g></svg>\n');

// 4. frames for gif, apng and animated webp
const FPS = 24, SECS = 3.4, N = Math.round(FPS * SECS);
const ease = (t) => 1 - Math.pow(1 - Math.min(1, Math.max(0, t)), 3);
const frameSvg = (t, bg, fill, w, h) => {
  const body = cells.map((c) => {
    const k = (t - c.delay) / c.dur; if (k <= 0) return '';
    const e = ease(k);
    return rect(c, -c.dx * (1 - e), Math.min(1, k * 1.8), fill);
  }).join('');
  const pad = 128;
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${word.x - pad} ${word.y - pad} ${word.w + 2 * pad} ${word.h + 2 * pad}" width="${w}" height="${h}">${bg ? `<rect x="${word.x - pad}" y="${word.y - pad}" width="${word.w + 2 * pad}" height="${word.h + 2 * pad}" fill="${bg}"/>` : ''}<g shape-rendering="crispEdges">${body}</g></svg>`;
};
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'anim-'));
const FW = 1200, FH = Math.round(FW * (word.h + 256) / (word.w + 256));
const render = (svg, w, file) => fs.writeFileSync(file, new Resvg(svg, { fitTo: { mode: 'width', value: w }, shapeRendering: 1 }).render().asPng());
for (const [name, bg, fill] of [['dark', V, I], ['transparent-white', null, I], ['transparent-primary', null, P]]) {
  const d = path.join(tmp, name); fs.mkdirSync(d);
  for (let f = 0; f < N; f++) render(frameSvg(f / FPS, bg, fill, FW, FH), FW, path.join(d, String(f).padStart(3, '0') + '.png'));
}
// email header: the logo is complete from the first frame (Outlook shows frame one only); blue pixels keep flowing along a line under it
const EW = 1200, EH = 320, eDir = path.join(tmp, 'email'); fs.mkdirSync(eDir);
const LOOP = 36;
for (let f = 0; f < LOOP; f++) {
  const dots = [0, 1, 2, 3, 4, 5, 6].map((i) => { const ph = ((f / LOOP) + i / 7) % 1; return `<rect x="${60 + ph * 1080}" y="276" width="14" height="14" fill="${P}" opacity="${(Math.sin(ph * Math.PI)).toFixed(2)}"/>`; }).join('');
  const lw = 640, lh = lw * word.h / word.w;
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${EW}" height="${EH}" viewBox="0 0 ${EW} ${EH}"><rect width="${EW}" height="${EH}" fill="${V}"/><svg x="${(EW - lw) / 2}" y="${(250 - lh) / 2}" width="${lw}" height="${lh}" viewBox="${word.x} ${word.y} ${word.w} ${word.h}"><path fill="${I}" shape-rendering="crispEdges" d="${word.d}"/></svg>${dots}</svg>`;
  render(svg, EW, path.join(eDir, String(f).padStart(3, '0') + '.png'));
}
const py = `
import sys,glob,os
from PIL import Image
tmp,out,fps,loopfps,hold=sys.argv[1],sys.argv[2],int(sys.argv[3]),int(sys.argv[4]),float(sys.argv[5])
def frames(d): return [Image.open(f).convert('RGBA') for f in sorted(glob.glob(os.path.join(tmp,d,'*.png')))]
def save_gif(name,fr,ms,loop=0):
    pal=[f.convert('RGB').convert('P',palette=Image.ADAPTIVE,colors=64) for f in fr]
    pal[0].save(os.path.join(out,name),save_all=True,append_images=pal[1:],duration=ms,loop=loop,optimize=True,disposal=2)
d=frames('dark'); save_gif('intro-pixel-stream-dark.gif',d,int(1000/fps))
for n in ('transparent-white','transparent-primary'):
    fr=frames(n)
    fr[0].save(os.path.join(out,'intro-pixel-stream-'+n.replace('transparent-','')+'.png'),save_all=True,append_images=fr[1:],duration=int(1000/fps),loop=0)
    fr[0].save(os.path.join(out,'intro-pixel-stream-'+n.replace('transparent-','')+'.webp'),save_all=True,append_images=fr[1:],duration=int(1000/fps),loop=0,lossless=True)
e=frames('email')
for f in e: pass
save_gif('header-animated.gif',e,int(1000/loopfps))
os.replace(os.path.join(out,'header-animated.gif'),os.path.join(os.path.dirname(os.path.dirname(out)),'brand','email','images','header-animated.gif')) if False else None
`;
execFileSync('python3', ['-c', py, tmp, out, String(FPS), '12', '1.0']);
fs.renameSync(path.join(out, 'header-animated.gif'), path.resolve('brand/email/images/header-animated.gif'));
fs.writeFileSync(path.join(out, 'README.md'), `# Animated logo files\n\n* intro-pixel-stream.svg and -primary.svg: pixels flow in from the left and settle into the wordmark once (CSS only, respects reduced motion).\n* loop-pixel-stream.svg: the same, repeating, for loaders and hero backgrounds.\n* intro-pixel-stream-dark.gif: on the dark color, for places that only accept GIF.\n* intro-pixel-stream-white.png / .webp and -primary.png / .webp: transparent animated PNG and WebP.\n* loader-streaming-pixels.svg, loader-mark-pulse.svg, intro-wordmark-reveal.svg: from the brand kit script.\n* ../email/images/header-animated.gif: the email header; the logo is complete in the first frame, blue pixels flow below it.\n`);
console.log('animations written:', fs.readdirSync(out).filter((f) => !f.endsWith('.md')).length, 'files in', out, '| frames', N);
