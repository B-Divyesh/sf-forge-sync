export function normalizeUrl(value) {
  return value.trim().replace(/\/+$/, '');
}

export function makeConfig({ org, kind, url, owner }) {
  const cleanOrg = org.trim();
  const cleanOwner = owner.trim();
  if (!/^[A-Za-z0-9_.-]+$/.test(cleanOrg)) throw new Error('Use a valid GitHub organization name.');
  if (!/^[A-Za-z0-9_.-]+$/.test(cleanOwner)) throw new Error('Use a valid target owner or namespace.');
  let parsed;
  try { parsed = new URL(normalizeUrl(url)); } catch { throw new Error('Enter a complete target URL, including https://.'); }
  if (parsed.protocol !== 'https:' && !['localhost', '127.0.0.1'].includes(parsed.hostname)) throw new Error('Use HTTPS for a remote target.');
  const targetKind = ['forgejo', 'codeberg', 'gitlab'].includes(kind) ? kind : 'forgejo';
  return `[source]
org = "${cleanOrg}"
token_env = "GITHUB_TOKEN"

[target]
kind = "${targetKind}"
base_url = "${normalizeUrl(url)}"
owner = "${cleanOwner}"
token_env = "FORGE_TOKEN"

[sync]
interval_seconds = 300
include_archived = true
private = true
state_dir = ".forge-sync"
archive_dir = "forge-archive"
git_archive = true
`;
}
