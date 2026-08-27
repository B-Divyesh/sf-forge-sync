import test from 'node:test';
import assert from 'node:assert/strict';
import { makeConfig, normalizeUrl } from '../config.js';

test('generates documented environment-only config', () => {
  const config = makeConfig({ org: 'acme', kind: 'gitlab', url: 'https://git.example/', owner: 'backup' });
  assert.match(config, /org = "acme"/); assert.match(config, /kind = "gitlab"/); assert.doesNotMatch(config, /pat_|secret/);
});
test('rejects unsafe remote URL and names', () => {
  assert.throws(() => makeConfig({ org: 'a b', kind: 'forgejo', url: 'http://example.com', owner: 'x' }));
});
test('normalizes trailing slashes', () => assert.equal(normalizeUrl('https://codeberg.org///'), 'https://codeberg.org'));
