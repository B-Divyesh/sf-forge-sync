import AxeBuilder from '@axe-core/playwright';
import { chromium } from 'playwright';
import { mkdir, writeFile } from 'node:fs/promises';

const base = process.argv[2] || 'http://127.0.0.1:4173';
const browser = await chromium.launch();
const context = await browser.newContext({
  viewport: { width: 390, height: 844 },
  reducedMotion: 'reduce'
});
const results = [];
for (const path of ['/', '/privacy/', '/terms/']) {
  const page = await context.newPage();
  await page.goto(new URL(path, base).href, { waitUntil: 'networkidle' });
  const focusFailures = [];
  for (let tab = 0; tab < 48; tab += 1) {
    await page.keyboard.press('Tab');
    const focus = await page.evaluate(() => {
      const element = document.activeElement;
      if (!element?.matches('a[href], button, input, select, summary, [tabindex]')) return null;
      const style = getComputedStyle(element);
      return {
        element: element.outerHTML.slice(0, 120),
        outlineStyle: style.outlineStyle,
        outlineWidth: Number.parseFloat(style.outlineWidth)
      };
    });
    if (focus && (focus.outlineStyle === 'none' || focus.outlineWidth < 3)) focusFailures.push(focus.element);
  }
  if (focusFailures.length) {
    throw new Error(`reduced-motion visible-focus failure on ${path}: ${focusFailures.join(', ')}`);
  }
  const report = await new AxeBuilder({ page }).analyze();
  results.push({ path, violations: report.violations });
  await page.close();
}
await context.close();
await browser.close();
await mkdir('.factory/evidence', { recursive: true });
await writeFile('.factory/evidence/axe.json', JSON.stringify(results, null, 2));
const serious = results.flatMap(result => result.violations).filter(item => ['serious', 'critical'].includes(item.impact));
console.log(`axe: ${results.reduce((sum, result) => sum + result.violations.length, 0)} total violations; ${serious.length} serious/critical`);
if (serious.length) {
  for (const item of serious) console.error(`${item.impact}: ${item.id} — ${item.help}`);
  process.exit(1);
}
