export interface AmbientRGB {
  r: number;
  g: number;
  b: number;
}

const cache = new Map<string, AmbientRGB>();
const STORAGE_PREFIX = 'pstream-ambient:';
const LAST_KEY = 'pstream-ambient:last';

export function getCachedAmbientColor(imageUrl: string): AmbientRGB | null {
  if (!imageUrl) return null;
  const inMem = cache.get(imageUrl);
  if (inMem) return inMem;

  try {
    const raw = localStorage.getItem(STORAGE_PREFIX + imageUrl);
    if (raw) {
      const parsed = JSON.parse(raw);
      cache.set(imageUrl, parsed);
      return parsed;
    }
  } catch {}
  return null;
}

export function getLastAmbientColor(): AmbientRGB | null {
  try {
    const raw = localStorage.getItem(LAST_KEY);
    if (raw) return JSON.parse(raw);
  } catch {}
  return null;
}

export function extractAmbientColor(imageUrl: string): Promise<AmbientRGB | null> {
  if (!imageUrl) return Promise.resolve(null);
  const cached = getCachedAmbientColor(imageUrl);
  if (cached) return Promise.resolve(cached);

  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';

    // Optimization: TMDB w92 thumbnail is only 2-4KB, downloads in ~15ms,
    // and produces the exact same color palette as a 5MB original backdrop.
    let fastUrl = imageUrl;
    if (fastUrl.includes('image.tmdb.org/t/p/')) {
      fastUrl = fastUrl.replace(/\/t\/p\/[^/]+/, '/t/p/w92');
    }
    img.src = fastUrl.includes('?') ? `${fastUrl}&cors=true` : `${fastUrl}?cors=true`;

    img.onload = () => {
      try {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        if (!ctx) return resolve(null);

        canvas.width = 16;
        canvas.height = 16;
        ctx.drawImage(img, 0, 0, 16, 16);
        const data = ctx.getImageData(0, 0, 16, 16).data;

        let r = 0, g = 0, b = 0, count = 0;
        let maxSat = 0, vibR = 0, vibG = 0, vibB = 0;

        for (let i = 0; i < data.length; i += 4) {
          const pr = data[i] ?? 0, pg = data[i + 1] ?? 0, pb = data[i + 2] ?? 0, pa = data[i + 3] ?? 0;
          if (pa <= 180) continue;
          const brightness = (pr * 299 + pg * 587 + pb * 114) / 1000;
          if (brightness <= 25 || brightness >= 230) continue;
          r += pr; g += pg; b += pb; count++;

          const max = Math.max(pr, pg, pb);
          const min = Math.min(pr, pg, pb);
          const sat = max === 0 ? 0 : (max - min) / max;
          if (sat > maxSat) { maxSat = sat; vibR = pr; vibG = pg; vibB = pb; }
        }

        if (count === 0) return resolve(null);

        let ar = Math.min(255, Math.round((r / count) * 0.45 + vibR * 0.55));
        let ag = Math.min(255, Math.round((g / count) * 0.45 + vibG * 0.55));
        let ab = Math.min(255, Math.round((b / count) * 0.45 + vibB * 0.55));

        // Saturation punch
        const maxCh = Math.max(ar, ag, ab);
        const boostFactor = maxCh > 60 ? 1.15 : 1.0;
        ar = Math.min(255, Math.round(ar * (ar === maxCh ? boostFactor : 1)));
        ag = Math.min(255, Math.round(ag * (ag === maxCh ? boostFactor : 1)));
        ab = Math.min(255, Math.round(ab * (ab === maxCh ? boostFactor : 1)));

        // Luminance floor
        const lum = (ar * 299 + ag * 587 + ab * 114) / 1000;
        if (lum < 55) {
          const scale = 55 / Math.max(lum, 1);
          ar = Math.min(255, Math.round(ar * scale));
          ag = Math.min(255, Math.round(ag * scale));
          ab = Math.min(255, Math.round(ab * scale));
        }

        const rgb = { r: ar, g: ag, b: ab };
        cache.set(imageUrl, rgb);
        try {
          localStorage.setItem(STORAGE_PREFIX + imageUrl, JSON.stringify(rgb));
          localStorage.setItem(LAST_KEY, JSON.stringify(rgb));
        } catch {}
        resolve(rgb);
      } catch {
        resolve(null);
      }
    };
    img.onerror = () => resolve(null);
  });
}
