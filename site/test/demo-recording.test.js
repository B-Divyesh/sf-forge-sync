import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';

test('@claim:demo-recording-current-output', () => {
  const result = spawnSync(process.execPath, ['scripts/generate-demo-recording.mjs', '--check'], {
    encoding: 'utf8'
  });
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
  assert.match(result.stdout, /Demo recording matches current output/);
});

test('demo pages use the generated recording and captured transcript', async () => {
  const transcript = await readFile('site/public/demo-transcript.txt', 'utf8');
  const pages = await Promise.all([
    readFile('site/index.html', 'utf8'),
    readFile('site/demo/index.html', 'utf8')
  ]);
  assert.match(transcript, /^\$ forge-sync demo\nCompleted sample mirror/);
  assert.match(transcript, /links between GitHub and target records: 1/);
  assert.match(transcript, /dated run history entries: 3/);
  for (const page of pages) {
    assert.match(page, /src="\/demo-recording\.svg"/);
    assert.match(page, /__DEMO_TRANSCRIPT__/);
    assert.doesNotMatch(page, /Example forge-sync demo terminal output|Sample demo output/);
  }
});
