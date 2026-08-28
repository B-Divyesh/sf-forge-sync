import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

test('@claim:minimum-rust-build', async () => {
  const cargo = await readFile('Cargo.toml', 'utf8');
  const readme = await readFile('README.md', 'utf8');
  assert.match(cargo, /rust-version = "1\.88"/);
  assert.match(readme, /Rust 1\.88 or newer/);

  const install = spawnSync('rustup', ['toolchain', 'install', '1.88.0', '--profile', 'minimal'], {
    encoding: 'utf8'
  });
  assert.equal(install.status, 0, install.stderr);

  const target = await mkdtemp(join(tmpdir(), 'forge-sync-rust-1.88-'));
  try {
    for (const command of [['build', '--locked'], ['test', '--locked']]) {
      const result = spawnSync('cargo', ['+1.88.0', ...command], {
        encoding: 'utf8',
        env: { ...process.env, CARGO_TARGET_DIR: target }
      });
      assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
    }
  } finally {
    await rm(target, { recursive: true, force: true });
  }
});
