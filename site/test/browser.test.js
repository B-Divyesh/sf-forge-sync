import test, { after, before } from 'node:test';
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { chromium } from 'playwright';

const base = 'http://127.0.0.1:4173';
let server;
let browser;

async function waitForServer() {
  for (let retry = 0; retry < 50; retry += 1) {
    try { if ((await fetch(base)).ok) return; } catch {}
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw new Error('Vite test server did not start');
}
before(async () => {
  server = spawn(process.execPath, ['node_modules/vite/bin/vite.js', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', '4173'], { stdio: 'ignore' });
  await waitForServer();
  browser = await chromium.launch();
});
after(async () => { await browser?.close(); server?.kill('SIGTERM'); });

test('@claim:demo-browser-isolation', async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const requests = [];
  page.on('request', request => requests.push(request.url()));
  await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
  await assert.doesNotReject(() => page.getByText('Demo — sample data, nothing is saved.').waitFor());
  await assert.doesNotReject(() => page.getByRole('button', { name: 'Reset demo' }).waitFor());
  const storageBefore = await page.evaluate(() => Object.keys(localStorage));
  assert.deepEqual(storageBefore, ['demo:forge-sync:session']);
  await page.getByRole('button', { name: 'Reset demo' }).click();
  const storageAfter = await page.evaluate(() => Object.keys(localStorage));
  assert.deepEqual(storageAfter, ['demo:forge-sync:session']);
  assert.ok(requests.every(url => new URL(url).origin === base), requests.join('\n'));
  await context.close();
});

test('@claim:configuration-has-no-token-field', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests = [];
  page.on('request', request => requests.push(request.url()));
  await page.goto(base, { waitUntil: 'networkidle' });
  await page.locator('#org').fill('harbor-coop');
  await page.locator('#kind').selectOption('gitlab');
  await page.locator('#url').fill('https://gitlab.example');
  await page.locator('#owner').fill('mirror');
  const config = await page.locator('#config-output').textContent();
  assert.match(config, /token_env = "GITHUB_TOKEN"/);
  assert.doesNotMatch(config, /token\s*=|secret|pat_/i);
  assert.equal(await page.locator('input[type="password"]').count(), 0);
  assert.ok(requests.every(url => new URL(url).origin === base), requests.join('\n'));
  await context.close();
});

test('@claim:offline-demo-after-first-visit', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload({ waitUntil: 'networkidle' });
  await context.setOffline(true);
  await page.reload({ waitUntil: 'domcontentloaded' });
  await assert.doesNotReject(() => page.getByText('Demo — sample data, nothing is saved.').waitFor());
  await assert.doesNotReject(() => page.getByText('You’re offline. The sample and configuration builder remain available after a first visit.').waitFor());
  await context.close();
});

test('routes, metadata, focus, and phone first action work', async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, reducedMotion: 'reduce' });
  const page = await context.newPage();
  for (const [path, title] of [['/', 'forge-sync — mirror a GitHub organization'], ['/demo/', 'Demo — forge-sync'], ['/privacy/', 'Privacy — forge-sync'], ['/terms/', 'Terms — forge-sync'], ['/404.html', 'Page not found — forge-sync']]) {
    await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
    assert.equal(await page.title(), title);
    assert.equal(await page.locator('html').getAttribute('lang'), 'en');
    assert.equal(await page.locator('main').count(), 1);
    assert.equal(await page.locator('h1').count(), 1);
    assert.ok(await page.locator('link[rel="canonical"]').count());
  }
  await page.goto(base, { waitUntil: 'networkidle' });
  assert.ok(await page.getByRole('link', { name: 'Try it with sample data' }).first().isVisible());
  await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
  assert.equal(await page.title(), 'Demo — forge-sync');
  await page.goto(base, { waitUntil: 'networkidle' });
  await page.getByRole('link', { name: 'Build your configuration' }).click();
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'configure-title');
  await page.goBack();
  await page.waitForTimeout(80);
  assert.equal(page.url(), `${base}/`);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'hero-title');
  await context.close();
});
