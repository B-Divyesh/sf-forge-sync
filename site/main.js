import './fonts.css';
import './style.css';
import './accessibility.css';
import { makeConfig } from './config.js';

const $ = (selector, root = document) => root.querySelector(selector);
const form = $('#config-form');
const output = $('#config-output');
const status = $('#config-status');

function renderConfig() {
  if (!form) return;
  try {
    const data = Object.fromEntries(new FormData(form));
    output.textContent = makeConfig(data);
    status.textContent = 'Configuration ready. Tokens stay in environment variables.';
    status.dataset.kind = 'success';
  } catch (error) {
    output.textContent = '# Complete the fields above to generate a safe configuration.';
    status.textContent = error.message;
    status.dataset.kind = 'error';
  }
}
form?.addEventListener('input', renderConfig);
form?.addEventListener('submit', (event) => event.preventDefault());
renderConfig();

$('#copy-config')?.addEventListener('click', async (event) => {
  try {
    await navigator.clipboard.writeText(output.textContent);
    event.currentTarget.textContent = 'Copied';
    setTimeout(() => { event.currentTarget.textContent = 'Copy config'; }, 1600);
  } catch {
    status.textContent = 'Clipboard access was blocked. Select the configuration and copy it manually.';
    status.dataset.kind = 'error';
  }
});

$('#download-config')?.addEventListener('click', () => {
  const url = URL.createObjectURL(new Blob([output.textContent], { type: 'text/plain' }));
  const link = document.createElement('a'); link.href = url; link.download = 'forge-sync.toml'; link.click(); URL.revokeObjectURL(url);
});

const licenseKey = 'sb_license:forge-sync';
const verdictKey = `${licenseKey}:verdict`;
const params = new URLSearchParams(location.search);
if (params.has('license')) {
  localStorage.setItem(licenseKey, params.get('license'));
  params.delete('license');
  history.replaceState({}, '', `${location.pathname}${params.size ? `?${params}` : ''}${location.hash}`);
}

function setLicenseState(valid, message) {
  document.documentElement.dataset.licensed = valid ? 'true' : 'false';
  const note = $('#license-status'); if (note) note.textContent = message;
  const download = $('#download-kit'); if (download) download.hidden = !valid;
}

async function verifyLicense(force = false) {
  const token = localStorage.getItem(licenseKey); if (!token) return;
  let cached;
  try { cached = JSON.parse(localStorage.getItem(verdictKey) || 'null'); } catch { cached = null; }
  if (cached?.valid) setLicenseState(true, 'Migration Kit unlocked on this device.');
  if (!force && cached && Date.now() - cached.checkedAt < 86_400_000) return;
  try {
    const response = await fetch(`https://api.sociobot.in/api/v1/products/forge-sync/verify?license=${encodeURIComponent(token)}`);
    if (!response.ok) throw new Error('verification unavailable');
    const verdict = await response.json(); localStorage.setItem(verdictKey, JSON.stringify({ valid: verdict.valid, checkedAt: Date.now() }));
    setLicenseState(verdict.valid, verdict.valid ? 'Migration Kit unlocked on this device.' : 'License no longer active. You can purchase a new license below.');
  } catch {
    setLicenseState(Boolean(cached?.valid), cached?.valid ? 'Offline — using the last valid license check.' : 'License check is unavailable. The free CLI remains fully available.');
  }
}
verifyLicense();

$('#license-form')?.addEventListener('submit', (event) => {
  event.preventDefault(); const token = new FormData(event.currentTarget).get('license').trim();
  if (!token) return; localStorage.setItem(licenseKey, token); localStorage.removeItem(verdictKey);
  setLicenseState(false, 'Checking license…'); verifyLicense(true);
});

$('#download-kit')?.addEventListener('click', () => {
  const checklist = `# forge-sync migration runbook\n\n## Before first sync\n- [ ] Create least-privilege source and target tokens\n- [ ] Run forge-sync doctor\n- [ ] Run a dry pass and review the target privacy setting\n- [ ] Back up .forge-sync/state.sqlite3\n\n## Cutover\n- [ ] Freeze repository creation during the final pass\n- [ ] Run forge-sync sync and confirm exit code 0\n- [ ] Compare branch and tag counts\n- [ ] Sample issue and PR discussion attribution\n- [ ] Update clone URLs and contribution documentation\n\n## Rollback\n- [ ] Keep GitHub read-only until acceptance is complete\n- [ ] Preserve forge-archive and the SQLite mapping database\n`;
  const url = URL.createObjectURL(new Blob([checklist], { type: 'text/markdown' }));
  const link = document.createElement('a'); link.href = url; link.download = 'forge-sync-migration-runbook.md'; link.click(); URL.revokeObjectURL(url);
});

window.addEventListener('offline', () => { const banner = $('#network-state'); if (banner) { banner.hidden = false; banner.textContent = 'You’re offline. Docs and the config builder still work; license checks will resume later.'; } });
window.addEventListener('online', () => { const banner = $('#network-state'); if (banner) banner.hidden = true; verifyLicense(true); });
if (!navigator.onLine) {
  const banner = $('#network-state');
  if (banner) { banner.hidden = false; banner.textContent = 'You’re offline. Docs and the config builder still work; license checks will resume later.'; }
}

if ('serviceWorker' in navigator) navigator.serviceWorker.register('/sw.js').catch(() => {});
