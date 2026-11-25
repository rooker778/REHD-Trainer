#!/usr/bin/env python3
"""Generate the neutral placeholder assets shipped with the public repo.

The release the author uses locally has FLiNG-style sounds/icon which are
not redistributable, so the public build ships these original assets.
Run this script if you want to regenerate them:

    python tools/make_assets.py
"""
import math
import struct
import wave
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
ASSETS.mkdir(exist_ok=True)


def make_icon() -> None:
    size = 256
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.rounded_rectangle((8, 8, size - 8, size - 8), radius=52, fill=(17, 21, 19, 255))
    d.rounded_rectangle((8, 8, size - 8, size - 8), radius=52, outline=(120, 200, 120, 255), width=7)
    font = ImageFont.truetype("C:/Windows/Fonts/arialbd.ttf", 104)
    text = "RE"
    box = d.textbbox((0, 0), text, font=font)
    w, h = box[2] - box[0], box[3] - box[1]
    d.text(((size - w) / 2 - box[0], (size - h) / 2 - box[1] - 4), text, font=font, fill=(120, 200, 120, 255))
    img.save(ASSETS / "trainer.ico", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])


def make_tone(path: Path, f0: float, f1: float, dur: float = 0.14, rate: int = 11025) -> None:
    n = int(rate * dur)
    frames = bytearray()
    phase = 0.0
    for i in range(n):
        t = i / max(1, n - 1)
        freq = f0 + (f1 - f0) * t
        phase += 2.0 * math.pi * freq / rate
        env = min(1.0, i / (rate * 0.005)) * math.exp(-3.2 * t)
        sample = int(9000 * env * math.sin(phase))
        frames += struct.pack("<h", sample)
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(rate)
        w.writeframes(bytes(frames))


make_icon()
make_tone(ASSETS / "toggle_enable.wav", 600.0, 1100.0)
make_tone(ASSETS / "toggle_disable.wav", 900.0, 420.0)
print("assets written to", ASSETS)
