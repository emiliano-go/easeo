"""Concatenate all markdown docs into docs/llms-full.txt.

Run before `zensical build` (or standalone) to produce a single plain-text
file containing the full documentation, suitable for LLM context ingestion.
"""

from __future__ import annotations

import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DOCS_DIR = PROJECT_ROOT / "docs"
OUTPUT = DOCS_DIR / "llms-full.txt"
SITE_URL = "https://easeo.emiliano-go.com"

SKIP_FILES = {"llms.txt", "llms-full.txt"}
SKIP_DIRS = {"overrides", "stylesheets", "assets"}


def route_path_from_file(filepath: Path) -> str:
    rel = filepath.relative_to(DOCS_DIR)
    parts = list(rel.parts)
    if parts[-1] == "index.md":
        parts.pop()
        return "/" + "/".join(parts) + "/" if parts else "/"
    stem = Path(*parts).with_suffix("")
    return "/" + str(stem) + "/"


def strip_frontmatter(text: str) -> str:
    if text.startswith("---"):
        parts = text.split("---", 2)
        if len(parts) >= 3:
            return parts[2].lstrip("\n")
    return text


def main() -> int:
    md_files = sorted(DOCS_DIR.rglob("*.md"))
    md_files = [
        f
        for f in md_files
        if f.name not in SKIP_FILES and not any(d in f.parts for d in SKIP_DIRS)
    ]

    sections = [
        "# easeo Documentation",
        "> Full documentation for easeo: deterministic SEO payload generation",
        f"> Source: {SITE_URL}",
        f"> Pages: {len(md_files)}",
        "",
    ]

    for md_file in md_files:
        route = route_path_from_file(md_file)
        url = SITE_URL.rstrip("/") + route
        body = strip_frontmatter(md_file.read_text(encoding="utf-8")).strip()

        sections.append("=" * 72)
        sections.append(f"PAGE: {url}")
        sections.append("=" * 72)
        sections.append("")
        sections.append(body)
        sections.append("")

    OUTPUT.write_text("\n".join(sections), encoding="utf-8")
    size_kb = OUTPUT.stat().st_size / 1024
    print(f"Written {OUTPUT} ({len(md_files)} pages, {size_kb:.0f} KB)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
