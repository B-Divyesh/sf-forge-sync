import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('Static Web Apps config delivers immutable assets and a restrictive Permissions-Policy', async () => {
  const config = JSON.parse(await readFile(new URL('../public/staticwebapp.config.json', import.meta.url)));
  assert.equal(config.globalHeaders['Permissions-Policy'], 'camera=(), microphone=(), geolocation=()');
  assert.match(config.globalHeaders['Content-Security-Policy'], /default-src 'self'/);
  const assets = config.routes.find(route => route.route === '/assets/*');
  assert.equal(assets.headers['Cache-Control'], 'public, max-age=31536000, immutable');
});
