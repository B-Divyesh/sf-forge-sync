import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { execFileSync } from 'node:child_process';

const buildId = process.env.VITE_BUILD_ID || execFileSync('git', ['rev-parse', '--short=8', 'HEAD'], { encoding: 'utf8' }).trim();

export default defineConfig({
  root: resolve(import.meta.dirname),
  publicDir: 'public',
  build: {
    outDir: resolve(import.meta.dirname, '../dist/site'),
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        main: resolve(import.meta.dirname, 'index.html'),
        demo: resolve(import.meta.dirname, 'demo/index.html'),
        privacy: resolve(import.meta.dirname, 'privacy/index.html'),
        terms: resolve(import.meta.dirname, 'terms/index.html'),
        notFound: resolve(import.meta.dirname, '404.html')
      }
    }
  },
  plugins: [{
    name: 'forge-sync-build-id',
    transformIndexHtml: html => html.replaceAll('__BUILD_ID__', buildId)
  }]
});
