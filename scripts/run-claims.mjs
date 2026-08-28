import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';

const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
assert.ok(claims.length > 0, 'claims.json must not be empty');
assert.equal(new Set(claims.map(claim => claim.id)).size, claims.length, 'claim IDs must be unique');

const sourceFiles = [
  ...(await readdir('site/test')).map(name => `site/test/${name}`),
  ...(await readdir('tests')).map(name => `tests/${name}`)
];
const source = (await Promise.all(sourceFiles.map(file => readFile(file, 'utf8')))).join('\n');

for (const claim of claims) {
  const tag = `@claim:${claim.id}`;
  assert.equal(source.split(tag).length - 1, 1, `${tag} must appear in exactly one test`);
  process.stdout.write(`\n== ${tag}: ${claim.test}\n`);
  const result = spawnSync(claim.test, { shell: true, stdio: 'inherit', env: process.env });
  if (result.status !== 0) process.exit(result.status ?? 1);
}

console.log(`\nPassed ${claims.length} claim tests.`);
