import AxeBuilder from '@axe-core/playwright';
import { chromium } from 'playwright';
import { mkdir, writeFile } from 'node:fs/promises';

const base = process.argv[2] || 'http://127.0.0.1:4173';
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const results = [];
for (const path of ['/', '/privacy/', '/terms/']) {
  const page = await context.newPage();
  await page.goto(new URL(path, base).href, { waitUntil: 'networkidle' });
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
