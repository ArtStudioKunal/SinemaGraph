#!/usr/bin/env python3
"""Generate all desktop icon formats from the Sinemagraph artwork.
The points were traced from the project owner's supplied 1024x1024 black/white logo.
This is a build asset generator, not a dependency at application runtime.
"""
from pathlib import Path
from PIL import Image, ImageDraw
import shutil

ROOT = Path(__file__).resolve().parents[2]
ICONS = ROOT / "assets" / "app-icon"
WEB = ROOT / "apps" / "filmcraft-web" / "web"
POLYGONS = [
 [(123,318),(123,898),(125,899),(318,706),(899,706),(900,705),(900,318),(899,317),(124,317)],
 [(801,123),(706,123),(704,124),(609,219),(609,220),(611,221),(614,220),(701,220),(702,221),(707,220),(802,125)],
 [(607,123),(512,123),(510,124),(415,219),(415,220),(417,221),(420,220),(513,220),(517,215),(608,125)],
 [(413,123),(318,123),(316,124),(221,219),(221,220),(223,221),(225,220),(305,220),(306,221),(309,220),(314,220),(315,221),(316,220),(319,220),(319,219),(322,217),(322,216),(327,212),(329,209),(414,125)],
 [(219,123),(124,123),(123,124),(123,219),(124,220),(219,126),(220,124)],
 [(900,125),(898,124),(803,219),(807,221),(809,220),(819,220),(820,221),(822,220),(891,220),(892,221),(894,220),(900,220)],
]

def generate():
    # 2x antialiasing retains crisp diagonals in 16px and taskbar icons.
    big = Image.new("RGB", (2048, 2048), "white")
    pen = ImageDraw.Draw(big)
    for points in POLYGONS:
        pen.polygon([(x * 2, y * 2) for x, y in points], fill="black")
    logo = big.resize((1024, 1024), Image.Resampling.LANCZOS)
    ICONS.mkdir(parents=True, exist_ok=True)
    logo.save(ICONS / "filmcraft-1024.png")
    # Replaces every runtime icon without breaking existing include_bytes! paths.
    logo.save(ICONS / "filmcraft.ico", sizes=[(16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)])
    mac = Image.new("RGBA", (1024, 1024), (0,0,0,0))
    mac.alpha_composite(logo.resize((824,824),Image.Resampling.LANCZOS).convert("RGBA"), (100,100))
    mac.resize((512,512),Image.Resampling.LANCZOS).save(ICONS / "filmcraft-macos-512.png")
    try:
        mac.save(ICONS / "filmcraft.icns")
    except (OSError, ValueError):
        pass
    for size in (16, 24, 32, 48, 64, 128, 256, 512):
        path = ICONS / "hicolor" / f"{size}x{size}" / "apps"
        path.mkdir(parents=True, exist_ok=True)
        logo.resize((size,size),Image.Resampling.LANCZOS).save(path / "ai.storyteller.filmcraft.png")
    WEB.mkdir(parents=True, exist_ok=True)
    logo.resize((128,128),Image.Resampling.LANCZOS).save(WEB / "favicon.png")
    source_svg = ROOT / "tools" / "sinemagraph" / "sinemagraph-icon.svg"
    shutil.copy2(source_svg, ICONS / "filmcraft.svg")
    shutil.copy2(source_svg, ICONS / "filmcraft-small.svg")
    shutil.copy2(source_svg, ICONS / "hicolor" / "scalable" / "apps" / "ai.storyteller.filmcraft.svg")
    print("Sinemagraph icon resources generated.")

if __name__ == "__main__":
    generate()
