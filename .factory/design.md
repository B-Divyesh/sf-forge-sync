# forge-sync visual thesis

## Direction: glacial minimal ceramics

forge-sync moves a living body of work without making it feel copied or brittle. The visual system therefore treats repositories as hand-thrown porcelain vessels crossing a dark, cold channel: pale mineral surfaces, hairline cobalt mapping marks, and a single ember glaze showing that the mirror is alive. It is quiet and exact, not a generic developer-dashboard neon treatment.

The landing page is deliberately single-mode. A warm glacial light canvas makes the ceramic illustration and code samples feel materially related, while the CLI itself remains native to the terminal.

## Tokens

- `ice-0 #f6f7f3`: page background, like overcast snow rather than pure white.
- `clay-1 #ebece5`: raised ceramic surface.
- `clay-2 #d5d8d0`: rules and inactive controls.
- `ink #172322`: primary text (contrast 14.4:1 on `ice-0`).
- `fjord #394d4b`: secondary text (contrast 8.2:1 on `ice-0`).
- `cobalt #174d67`: links, focus, active paths (contrast 8.7:1 on `ice-0`).
- `ember #a53f26`: the live-sync accent and destructive/warning edge (contrast 6.1:1 on `ice-0`).
- `moss #2f6250`: success (contrast 6.5:1 on `ice-0`).
- `night #10201f`: terminal/code surface; text is `#f3f4ee` (contrast 14.5:1).

## Type and spacing

Headings use the self-hosted variable serif **Fraunces**, chosen for pinched, vessel-like letterforms. Interface text and code use the self-hosted variable sans **IBM Plex Sans**, whose engineered shapes keep configuration and status output legible. Fonts are subset WOFF2, swap-rendered, and capped to two files.

The spacing rhythm is 4/8px based: 4, 8, 12, 16, 24, 32, 48, 72, and 112px. Reading measure stays between 48–72 characters. The page uses broad unboxed fields, then thin rules only where a boundary carries meaning. Controls are at least 44px tall.

## Interaction grammar and motion

The sync path is the core interaction motif: state changes draw from source to archive to target. Entrances use a 220ms opacity + 8px vertical settle; accordion disclosure and copy feedback use 180ms opacity/transform. Nothing loops. With `prefers-reduced-motion`, transitions and smooth scrolling are removed and the complete path is rendered immediately. Hover is never the sole state indicator; focus uses a 3px cobalt ring with a 3px offset.

## Asset plan and provenance

- `site/ceramic-mirror.webp`: original AI-generated hero still showing two hand-thrown porcelain repository vessels connected by a fine cobalt path over a glacial ceramic plane. Generated for this product with the factory Azure `factory-image` deployment via `/opt/fleet/lib/gen-image.sh`, 27 August 2026. Prompt is preserved in `site/public/ceramic-mirror.prompt.json`. No source image, trademark, embedded text, or third-party asset was used. Product-owned output, used under the factory's generated-asset terms.
- `site/public/og-image.webp` and `site/public/apple-touch-icon.png`: product-owned crops derived locally from `site/ceramic-mirror.webp`; no new source asset or third-party material.
- Directional marks, wordmark, status dots, and diagram lines are original CSS/SVG primitives authored in-repo; no icon library is shipped.

The hero image explains the product's world (a precise transfer between durable stores), while live metadata counts and the configuration composer explain the actual operation. No decorative stock imagery is used.
