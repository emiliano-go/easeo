"""Generate the documentation social card.

Writes a 1200x630 PNG used as ``default_og_image`` for the docs site. Run it
when the wordmark, tagline, or palette changes:

    python scripts/generate_og_image.py

Requires Pillow. The output path is ``docs/assets/og-image.png``.
"""

from __future__ import annotations

import sys
from pathlib import Path

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:  # pragma: no cover
    print("Pillow is required: pip install pillow", file=sys.stderr)
    raise

PROJECT_ROOT = Path(__file__).resolve().parent.parent
OUTPUT = PROJECT_ROOT / "docs" / "assets" / "og-image.png"

WIDTH, HEIGHT = 1200, 630
BACKGROUND = (14, 17, 22)
ACCENT = (205, 75, 251)
FOREGROUND = (245, 245, 250)
MUTED = (150, 153, 170)

FONT_CANDIDATES = {
    "bold": [
        "/usr/share/fonts/gsfonts/NimbusSans-Bold.otf",
        "/usr/share/fonts/Adwaita/AdwaitaSans-Regular.ttf",
    ],
    "regular": [
        "/usr/share/fonts/gsfonts/NimbusSans-Regular.otf",
        "/usr/share/fonts/Adwaita/AdwaitaSans-Regular.ttf",
    ],
}


def _font(kind: str, size: int) -> ImageFont.FreeTypeFont:
    for path in FONT_CANDIDATES[kind]:
        if Path(path).exists():
            return ImageFont.truetype(path, size)
    return ImageFont.load_default(size)


def _centered(draw: ImageDraw.ImageDraw, y: int, text: str, font, fill) -> int:
    left, top, right, bottom = draw.textbbox((0, 0), text, font=font)
    draw.text(((WIDTH - (right - left)) / 2 - left, y), text, font=font, fill=fill)
    return bottom - top


def main() -> int:
    image = Image.new("RGB", (WIDTH, HEIGHT), BACKGROUND)
    draw = ImageDraw.Draw(image)

    # Accent bar across the top and a subtle bottom rule.
    draw.rectangle([0, 0, WIDTH, 10], fill=ACCENT)
    draw.rectangle([0, HEIGHT - 4, WIDTH, HEIGHT], fill=(30, 33, 42))

    wordmark = _font("bold", 168)
    _centered(draw, 150, "easeo", wordmark, ACCENT)

    tagline = _font("regular", 46)
    _centered(draw, 360, "Deterministic SEO metadata generation", tagline, FOREGROUND)

    stack = _font("regular", 30)
    _centered(draw, 440, "Rust core / Python / JavaScript", stack, MUTED)

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    image.save(OUTPUT, "PNG", optimize=True)
    size_kb = OUTPUT.stat().st_size / 1024
    print(f"Wrote {OUTPUT} ({WIDTH}x{HEIGHT}, {size_kb:.0f} KB)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
