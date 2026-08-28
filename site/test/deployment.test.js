import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('Static Web Apps config delivers immutable assets and a restrictive Permissions-Policy', async () => {
  const config = JSON.parse(await readFile(new URL('../public/staticwebapp.config.json', import.meta.url)));
  assert.equal(config.globalHeaders['Permissions-Policy'], 'camera=(), microphone=(), geolocation=()');
  assert.match(config.globalHeaders['Content-Security-Policy'], /default-src 'self'/);
  const assets = config.routes.find(route => route.route === '/assets/*');
  assert.equal(assets.headers['Cache-Control'], 'public, max-age=31536000, immutable');
  assert.deepEqual(config.responseOverrides['404'], { rewrite: '/404.html', statusCode: 404 });
});

test('crawl surfaces list every route and no dead checkout remains', async () => {
  const sitemap = await readFile(new URL('../public/sitemap.xml', import.meta.url), 'utf8');
  for (const route of ['/', '/demo/', '/privacy/', '/terms/', '/404']) {
    assert.ok(sitemap.includes(`https://forge-sync.sociobot.in${route}`), route);
  }
  const pages = await Promise.all([
    '../index.html', '../demo/index.html', '../privacy/index.html', '../terms/index.html', '../404.html'
  ].map(path => readFile(new URL(path, import.meta.url), 'utf8')));
  assert.doesNotMatch(pages.join('\n'), /products\/forge-sync\/checkout|Buy Migration Kit/);
  assert.match(await readFile(new URL('../public/robots.txt', import.meta.url), 'utf8'), /Sitemap: https:\/\/forge-sync\.sociobot\.in\/sitemap\.xml/);
});
