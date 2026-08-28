import test, { after, before } from 'node:test';
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { readFile } from 'node:fs/promises';
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
  await page.goto(base, { waitUntil: 'networkidle' });
  await page.evaluate(() => localStorage.setItem('real:forge-sync:sentinel', 'keep'));
  assert.equal(await page.locator('#demo-banner').isVisible(), false);
  await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
  await assert.doesNotReject(() => page.getByText('Demo — sample data, nothing is saved.').waitFor());
  await assert.doesNotReject(() => page.getByRole('button', { name: 'Reset demo' }).waitFor());
  assert.equal(await page.title(), 'Demo — forge-sync');
  assert.equal(await page.locator('link[rel="canonical"]').getAttribute('href'), 'https://forge-sync.sociobot.in/?demo=1');
  assert.match(await page.locator('#demo-panel').innerText(), /harbor-tools[\s\S]*pull-request record[\s\S]*JSON archive/);
  const storageBefore = await page.evaluate(() => Object.keys(localStorage));
  assert.deepEqual(storageBefore.sort(), ['demo:forge-sync:session', 'real:forge-sync:sentinel']);
  const sessionBefore = await page.evaluate(() => localStorage.getItem('demo:forge-sync:session'));
  await page.getByRole('button', { name: 'Reset demo' }).click();
  const storageAfter = await page.evaluate(() => Object.keys(localStorage));
  assert.deepEqual(storageAfter.sort(), ['demo:forge-sync:session', 'real:forge-sync:sentinel']);
  assert.equal(await page.evaluate(() => localStorage.getItem('real:forge-sync:sentinel')), 'keep');
  assert.notEqual(await page.evaluate(() => localStorage.getItem('demo:forge-sync:session')), sessionBefore);
  await page.getByRole('link', { name: 'Leave demo and build configuration' }).first().click();
  await page.waitForLoadState('networkidle');
  assert.equal(page.url(), `${base}/`);
  assert.equal(await page.locator('#demo-banner').isVisible(), false);
  assert.deepEqual(await page.evaluate(() => Object.keys(localStorage)), ['real:forge-sync:sentinel']);
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
  await page.locator('#org').fill('offline-harbor');
  assert.match(await page.locator('#config-output').textContent(), /org = "offline-harbor"/);
  await context.close();
});

test('@claim:website-no-tracking', async () => {
  const runtime = await readFile('site/main.js', 'utf8');
  assert.doesNotMatch(runtime, /sendBeacon|XMLHttpRequest|\bfetch\s*\(|gtag\s*\(/);
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests = [];
  page.on('request', request => requests.push(request.url()));
  for (const path of ['/', '/?demo=1', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
  }
  await page.goto(base, { waitUntil: 'networkidle' });
  await page.locator('#org').fill('privacy-check');
  await page.locator('#kind').selectOption('codeberg');
  assert.ok(requests.length > 0);
  assert.ok(requests.every(url => new URL(url).origin === base), requests.join('\n'));
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
  const action = page.getByRole('link', { name: 'Try it with sample data' }).first();
  assert.ok(await action.isVisible());
  assert.ok((await action.boundingBox()).y + (await action.boundingBox()).height <= 844, 'sample action must fit in the first phone viewport');
  assert.equal(await page.locator('#demo-banner').isVisible(), false);
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth), 390);
  for (let index = 0; index < 10; index += 1) {
    await page.keyboard.press('Tab');
    const focus = await page.evaluate(() => {
      const node = document.activeElement;
      const style = getComputedStyle(node);
      const box = node.getBoundingClientRect();
      return { outline: Number.parseFloat(style.outlineWidth), width: box.width, height: box.height };
    });
    assert.ok(focus.outline >= 3, `keyboard focus ${index + 1} has no designed ring`);
    assert.ok(focus.width >= 1 && focus.height >= 1);
  }
  await page.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth), 390, '200% text must not create page-level horizontal overflow');
  await page.evaluate(() => { document.documentElement.style.fontSize = ''; });
  await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
  assert.equal(await page.title(), 'Demo — forge-sync');
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'demo-panel-title');
  assert.equal(await page.locator('h1').count(), 1);
  assert.equal(await page.locator('#demo-panel-title').evaluate(node => node.tagName), 'H1');
  assert.equal(await page.locator('#hero-title').evaluate(node => node.tagName), 'H2');
  const visibleHeadings = await page.locator('h1,h2,h3').evaluateAll(nodes => nodes
    .filter(node => !node.closest('[hidden]') && getComputedStyle(node).display !== 'none')
    .map(node => ({ tag: node.tagName, text: node.textContent.trim() })));
  assert.deepEqual(visibleHeadings[0], { tag: 'H1', text: 'See a completed sample mirror.' });
  const recording = page.locator('#demo-panel .demo-recording img');
  assert.ok(await recording.isVisible());
  assert.ok((await recording.boundingBox()).y < 844, 'the current-command recording must begin in the demo first screen');
  assert.match(await page.locator('#demo-panel .demo-recording code').textContent(), /links between GitHub and target records: 1/);
  assert.match(await page.locator('#route-status').textContent(), /See a completed sample mirror/);
  await page.goto(base, { waitUntil: 'networkidle' });
  await page.getByRole('link', { name: 'Try it with sample data' }).first().click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'demo-panel-title');
  await page.getByRole('link', { name: 'Leave demo and build configuration' }).first().click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'hero-title');
  await page.goBack();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'demo-panel-title');
  await page.goForward();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'hero-title');
  await page.getByRole('link', { name: 'Build your configuration' }).click();
  await page.waitForTimeout(20);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'configure-title');
  await page.goBack();
  await page.waitForTimeout(80);
  assert.equal(page.url(), `${base}/`);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'hero-title');
  await page.goForward();
  await page.waitForTimeout(80);
  assert.equal(await page.evaluate(() => document.activeElement?.id), 'configure-title');
  assert.ok(await page.evaluate(() => window.scrollY > 0));
  await context.close();

  const desktop = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const desktopPage = await desktop.newPage();
  await desktopPage.goto(base, { waitUntil: 'networkidle' });
  const factBoxes = await desktopPage.locator('.facts li').evaluateAll(nodes => nodes.map(node => {
    const box = node.getBoundingClientRect();
    return { text: node.textContent.trim(), bottom: box.bottom };
  }));
  assert.equal(factBoxes.length, 3);
  for (const fact of factBoxes) {
    assert.ok(fact.bottom <= 900, `${fact.text} ends below the 1440 × 900 first screen at ${fact.bottom}`);
  }
  await desktop.close();
});
