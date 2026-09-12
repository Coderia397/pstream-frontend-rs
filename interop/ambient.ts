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

        // Weighted blend favoring balanced atmospheric average
        let ar = Math.round((r / count) * 0.70 + vibR * 0.30);
        let ag = Math.round((g / count) * 0.70 + vibG * 0.30);
        let ab = Math.round((b / count) * 0.70 + vibB * 0.30);

        // Desaturate 35% toward neutral gray for an understated cinematic look
        const mean = (ar + ag + ab) / 3;
        ar = Math.round(ar * 0.65 + mean * 0.35);
        ag = Math.round(ag * 0.65 + mean * 0.35);
        ab = Math.round(ab * 0.65 + mean * 0.35);

        // Tone down intensity toward darker/blacker side (nudged one notch darker)
        ar = Math.min(255, Math.max(8, Math.round(ar * 0.72)));
        ag = Math.min(255, Math.max(8, Math.round(ag * 0.72)));
        ab = Math.min(255, Math.max(8, Math.round(ab * 0.72)));

        // Keep luminance in a deep, subtle dark range (between 18 and 48)
        const lum = (ar * 299 + ag * 587 + ab * 114) / 1000;
        if (lum > 48) {
          const scale = 48 / lum;
          ar = Math.round(ar * scale);
          ag = Math.round(ag * scale);
          ab = Math.round(ab * scale);
        } else if (lum < 18) {
          const scale = 18 / Math.max(lum, 1);
          ar = Math.round(ar * scale);
          ag = Math.round(ag * scale);
          ab = Math.round(ab * scale);
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
