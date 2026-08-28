import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';

test('@claim:mit-license', async () => {
  assert.match(await readFile('LICENSE', 'utf8'), /Permission is hereby granted/);
  assert.match(await readFile('Cargo.toml', 'utf8'), /license = "MIT"/);
});

test('each declared claim has exactly one tagged test and a runnable command', async () => {
  const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
  const files = [
    ...(await readdir('site/test')).map(name => `site/test/${name}`),
    ...(await readdir('tests')).map(name => `tests/${name}`)
  ];
  const source = (await Promise.all(files.map(file => readFile(file, 'utf8')))).join('\n');
  assert.ok(claims.length > 0);
  assert.equal(new Set(claims.map(claim => claim.id)).size, claims.length);
  for (const claim of claims) {
    assert.ok(claim.claim && claim.where && claim.test && claim.sandbox);
    assert.equal(source.split(`@claim:${claim.id}`).length - 1, 1, claim.id);
    assert.match(claim.test, /^npm run test:claim:/);
  }
});

test('all routes ship complete product metadata and the shared skeleton', async () => {
  for (const file of ['site/index.html', 'site/demo/index.html', 'site/privacy/index.html', 'site/terms/index.html', 'site/404.html']) {
    const html = await readFile(file, 'utf8');
    assert.match(html, /<html lang="en">/);
    assert.equal((html.match(/<h1\b/g) || []).length, 1, file);
    assert.equal((html.match(/<main\b/g) || []).length, 1, file);
    assert.match(html, /<title>[^<]+ — forge-sync<\/title>|<title>forge-sync — [^<]+<\/title>/, file);
    for (const marker of ['rel="canonical"', 'property="og:image"', 'property="og:image:width" content="1200"', 'property="og:image:height" content="630"', 'name="twitter:card"', 'rel="apple-touch-icon"', 'Built by Param Factory', 'href="/privacy/"', 'href="/terms/"']) {
      assert.ok(html.includes(marker), `${file} lacks ${marker}`);
    }
  }
});
