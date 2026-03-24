# rara Icon Specification

## Grid System

All product icons are built on a **512x512** viewBox with a shared squircle container.

```
viewBox:        512 x 512
container:      squircle (superellipse n≈5)
safe zone:      20% inset → 102px padding → 308x308 content area
optical center: (256, 240) — shifted up 3% for perceived centering
stroke width:   36 units (7% of viewBox)
stroke caps:    round
stroke joins:   round
```

## Zones

| Zone | Inset | Box | Use |
|------|-------|-----|-----|
| Bleed | 0% | 512x512 | Background fill only |
| Outer safe | 10% | 410x410 | Maximum extent |
| **Content** | **20%** | **308x308** | **Primary marks** |
| Optical center | — | (256, 240) | Perceived center |

## Squircle Container

Shared `clipPath` used by ALL product icons — never modify this shape:

```svg
<clipPath id="sq">
  <path d="M256 0C308 0,360 0,400 8C432 16,460 32,480 52C500 72,508
    96,510 128C512 160,512 208,512 256C512 304,512 352,510 384C508
    416,500 440,480 460C460 480,432 496,400 504C360 512,308 512,256
    512C204 512,152 512,112 504C80 496,52 480,32 460C12 440,4 416,2
    384C0 352,0 304,0 256C0 208,0 160,2 128C4 96,12 72,32 52C52
    32,80 16,112 8C152 0,204 0,256 0Z"/>
</clipPath>
```

## Color Assignment

Each product owns ONE color from the rara palette. The mark inside is always **white (#fff)**.

| Product | Color | Hex | Palette token |
|---------|-------|-----|---------------|
| rara | Coral Pink | #E8788A | color.pop |
| rara-trading | Peach Gold | #E8B86D | color.warn |
| tenki | Mint Green | #7BC4A0 | color.ok |
| kotoba | Lavender | #9BAFD9 | ansi.blue |

## Interior Mark Rules

1. **Geometric monogram** — construct from lines and arcs, not font glyphs
2. **Max 3-4 strokes** — must be recognizable as a silhouette
3. Stay inside the **308x308 content zone** (102,102)→(410,410)
4. Use `stroke-width="36"` and `stroke-linecap="round"`
5. All marks are **white on color** — no outlines, no gradients
6. Monochrome variants: mark in ink (#3D2B2E) on transparent

## Size Variants

| Size | File | Notes |
|------|------|-------|
| 512px | `{name}.svg` | Source / marketing |
| 64px | `{name}-64.png` | GitHub avatar, docs |
| 32px | `{name}-32.png` | Tab bar, sidebar |
| 16px | `{name}-16.png` | Favicon — simplify marks if needed |

At 16px: if the interior mark loses clarity, fall back to just the colored squircle.

## Adding a New Product Icon

1. Pick an unused color from the rara palette (ok, warn, ansi.* colors)
2. Design a 2-3 stroke geometric monogram of the product initial(s)
3. Place inside the shared squircle container at `stroke-width="36"`
4. Test at 16px — squint test must pass
5. Export SVG + PNG at 512/64/32/16
6. Add to `assets/icons/` and update the color table above
