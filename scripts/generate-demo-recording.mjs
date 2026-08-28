import { createHash } from 'node:crypto';
import { readFile, rm, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';

const checkOnly = process.argv.includes('--check');
const transcriptPath = 'site/public/demo-transcript.txt';
const svgPath = 'site/public/demo-recording.svg';

const result = spawnSync('cargo', ['run', '--quiet', '--', 'demo'], {
  encoding: 'utf8',
  env: process.env
});
if (result.status !== 0) {
  process.stderr.write(result.stderr);
  process.exit(result.status ?? 1);
}

const outputMatch = result.stdout.match(/^Demo output: (.+)$/m);
if (!outputMatch) throw new Error('forge-sync demo did not print its output directory');
const outputDirectory = outputMatch[1].trim();
const normalizedOutput = result.stdout
  .replace(`Demo output: ${outputDirectory}`, 'Demo output: /tmp/forge-sync-demo-…')
  .trimEnd();
const transcript = `$ forge-sync demo\n${normalizedOutput}\n`;
await rm(outputDirectory, { recursive: true, force: true });

const escapeXml = value => value
  .replaceAll('&', '&amp;')
  .replaceAll('<', '&lt;')
  .replaceAll('>', '&gt;');
const digest = createHash('sha256').update(transcript).digest('hex');
const lines = transcript.trimEnd().split('\n');
const text = lines.map((line, index) =>
  `  <text x="54" y="${112 + index * 38}" class="${index === 0 ? 'command' : 'output'}">${escapeXml(line)}</text>`
).join('\n');
const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="410" viewBox="0 0 1200 410" role="img" aria-labelledby="title description" data-transcript-sha256="${digest}">
  <title id="title">Captured forge-sync demo terminal output</title>
  <desc id="description">The current forge-sync demo command completes an isolated Harbor Cooperative sample mirror and prints its temporary output path.</desc>
  <rect width="1200" height="410" rx="10" fill="#10201f"/>
  <path d="M0 62H1200" stroke="#40504e"/>
  <circle cx="30" cy="31" r="7" fill="none" stroke="#78908b"/>
  <circle cx="54" cy="31" r="7" fill="none" stroke="#78908b"/>
  <circle cx="78" cy="31" r="7" fill="none" stroke="#78908b"/>
  <text x="1140" y="37" text-anchor="end" class="label">captured CLI output</text>
  <style>.command,.output,.label{font-family:ui-monospace,SFMono-Regular,Consolas,monospace}.command{font-size:20px;fill:#9bd2b4}.output{font-size:18px;fill:#f3f4ee}.label{font-size:15px;fill:#aebbb6;letter-spacing:1px}</style>
${text}
</svg>
`;

if (checkOnly) {
  const [savedTranscript, savedSvg] = await Promise.all([
    readFile(transcriptPath, 'utf8'),
    readFile(svgPath, 'utf8')
  ]);
  if (savedTranscript !== transcript) throw new Error('demo transcript differs from current forge-sync demo output');
  if (savedSvg !== svg) throw new Error('demo SVG differs from the generated current-command recording');
  process.stdout.write(`Demo recording matches current output (${digest}).\n`);
} else {
  await Promise.all([
    writeFile(transcriptPath, transcript),
    writeFile(svgPath, svg)
  ]);
  process.stdout.write(`Wrote ${transcriptPath} and ${svgPath} (${digest}).\n`);
}
