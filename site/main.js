import './fonts.css';
import './style.css';
import './repair.css';
import './accessibility.css';
import { makeConfig } from './config.js';

const $ = (selector, root = document) => root.querySelector(selector);
const demoPrefix = 'demo:forge-sync:';
const isDemo = location.pathname.startsWith('/demo') || new URLSearchParams(location.search).get('demo') === '1';
const newDemoSession = () => globalThis.crypto?.randomUUID?.() || `${Date.now()}-${Math.random()}`;

function setMeta(selector, value, attribute = 'content') {
  document.querySelector(selector)?.setAttribute(attribute, value);
}

function activateDemo() {
  if (!isDemo) return;
  document.body.dataset.demo = 'true';
  if (new URLSearchParams(location.search).get('demo') === '1') {
    const url = 'https://forge-sync.sociobot.in/?demo=1';
    const description = 'Inspect forge-sync’s isolated completed sample mirror before you use a real organization.';
    document.title = 'Demo — forge-sync';
    setMeta('meta[name="description"]', description);
    setMeta('meta[property="og:title"]', document.title);
    setMeta('meta[property="og:description"]', description);
    setMeta('meta[property="og:url"]', url);
    setMeta('meta[name="twitter:title"]', document.title);
    setMeta('meta[name="twitter:description"]', description);
    setMeta('link[rel="canonical"]', url, 'href');
  }
  localStorage.setItem(`${demoPrefix}session`, newDemoSession());
  $('#demo-banner')?.removeAttribute('hidden');
  const panel = $('#demo-panel');
  if (panel) panel.hidden = false;
}
function resetDemo() {
  for (let index = localStorage.length - 1; index >= 0; index -= 1) {
    const key = localStorage.key(index);
    if (key?.startsWith(demoPrefix)) localStorage.removeItem(key);
  }
  localStorage.setItem(`${demoPrefix}session`, newDemoSession());
  const note = $('#route-status');
  if (note) note.textContent = 'Demo reset. The sample data is new.';
}
activateDemo();
$('#reset-demo')?.addEventListener('click', resetDemo);
if (isDemo) {
  document.querySelectorAll('a[href="/"]').forEach(link => link.addEventListener('click', () => {
    for (let index = localStorage.length - 1; index >= 0; index -= 1) {
      const key = localStorage.key(index);
      if (key?.startsWith(demoPrefix)) localStorage.removeItem(key);
    }
  }));
}

const form = $('#config-form');
const output = $('#config-output');
const status = $('#config-status');
function renderConfig() {
  if (!form || !output || !status) return;
  try {
    output.textContent = makeConfig(Object.fromEntries(new FormData(form)));
    status.textContent = 'Configuration ready. Add token environment-variable names when you run the CLI.';
    status.dataset.kind = 'success';
  } catch (error) {
    output.textContent = '# Complete the fields above to generate the configuration.';
    status.textContent = error.message;
    status.dataset.kind = 'error';
  }
}
form?.addEventListener('input', renderConfig);
form?.addEventListener('submit', event => event.preventDefault());
renderConfig();

$('#copy-config')?.addEventListener('click', async event => {
  try {
    await navigator.clipboard.writeText(output.textContent);
    event.currentTarget.textContent = 'Configuration copied.';
    setTimeout(() => { event.currentTarget.textContent = 'Copy configuration'; }, 1600);
  } catch {
    status.textContent = 'Clipboard access was blocked. Select the configuration and copy it manually.';
    status.dataset.kind = 'error';
  }
});
$('#download-config')?.addEventListener('click', () => {
  const url = URL.createObjectURL(new Blob([output.textContent], { type: 'text/plain' }));
  const link = document.createElement('a');
  link.href = url; link.download = 'forge-sync.toml'; link.click();
  URL.revokeObjectURL(url);
});

function savePosition() {
  history.replaceState({ ...(history.state || {}), scrollY: window.scrollY }, '', location.href);
}
function focusDestination(target) {
  const heading = target.matches('h1,h2') ? target : (target.querySelector('h1') || target.querySelector('h2'));
  if (!heading) return;
  heading.setAttribute('tabindex', '-1');
  heading.focus({ preventScroll: true });
  const live = $('#route-status');
  if (live) live.textContent = `Moved to ${heading.textContent.trim()}`;
}
function goToHash(hash, push = true) {
  const target = $(hash);
  if (!target) return;
  savePosition();
  const destinationY = Math.max(0, window.scrollY + target.getBoundingClientRect().top);
  if (push) history.pushState({ scrollY: destinationY, hash }, '', `${location.pathname}${location.search}${hash}`);
  target.scrollIntoView({ block: 'start', behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
  setTimeout(() => focusDestination(target), 0);
}
document.addEventListener('click', event => {
  const link = event.target.closest('a[href^="#"]');
  if (!link || link.getAttribute('href') === '#') return;
  const hash = link.getAttribute('href');
  if (!$(hash)) return;
  event.preventDefault(); goToHash(hash);
});
history.scrollRestoration = 'manual';
history.replaceState({ ...(history.state || {}), scrollY: window.scrollY }, '', location.href);
window.addEventListener('popstate', event => {
  const y = event.state?.scrollY || 0;
  window.scrollTo({ top: y, behavior: 'auto' });
  const hash = location.hash;
  setTimeout(() => {
    if (hash && $(hash)) focusDestination($(hash));
    else if (y === 0) focusDestination($('#main'));
  }, 0);
});

if (location.hash && $(location.hash)) {
  requestAnimationFrame(() => focusDestination($(location.hash)));
}

function setOfflineBanner(show) {
  const banner = $('#network-state');
  if (!banner) return;
  banner.hidden = !show;
  banner.textContent = 'You’re offline. The sample and configuration builder remain available after a first visit.';
}
window.addEventListener('offline', () => setOfflineBanner(true));
window.addEventListener('online', () => setOfflineBanner(false));
if (!navigator.onLine) setOfflineBanner(true);
if ('serviceWorker' in navigator) navigator.serviceWorker.register('/sw.js').catch(() => {});
