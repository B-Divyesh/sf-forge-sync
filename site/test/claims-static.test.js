import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('@claim:mit-license', async () => {
  assert.match(await readFile('LICENSE', 'utf8'), /Permission is hereby granted/);
  assert.match(await readFile('Cargo.toml', 'utf8'), /license = "MIT"/);
});
