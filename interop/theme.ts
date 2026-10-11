function themeColorMeta(): HTMLMetaElement {
  let meta = document.querySelector('meta[name="theme-color"]') as HTMLMetaElement | null;
  if (!meta) {
    meta = document.createElement('meta');
    meta.name = 'theme-color';
    document.head.appendChild(meta);
  }
  return meta;
}

export function setThemeColor(r: number, g: number, b: number): void {
  try {
    themeColorMeta().content = `rgb(${Math.round(r * 0.45)}, ${Math.round(g * 0.45)}, ${Math.round(b * 0.45)})`;
  } catch {}
}

export function applyAmbientColor(r: number, g: number, b: number): void {
  try {
    const root = document.documentElement.style;
    root.setProperty('--ambient-r', String(r));
    root.setProperty('--ambient-g', String(g));
    root.setProperty('--ambient-b', String(b));
  } catch {}
  setThemeColor(r, g, b);
}
