#!/usr/bin/env python3
"""Hook alive.js into the app. Run from the repo root:  python3 apply_alive.py
Idempotent: adds <script src="alive.js" defer> to index.html, adds alive.js to the sw.js cache list, bumps the cache version."""
import re
h = open('index.html', encoding='utf-8').read()
if 'alive.js' not in h:
    h = h.replace('</body>', '<script src="alive.js" defer></script>\n</body>', 1)
    open('index.html', 'w', encoding='utf-8').write(h); print('index.html: script tag added')
else: print('index.html: already hooked')
w = open('sw.js', encoding='utf-8').read()
if 'alive.js' not in w:
    w = re.sub(r"(\n\s*)'index\.html',", r"\1'index.html',\1'alive.js',", w, count=1)
    w = re.sub(r"timetable-v(\d+)", lambda m: f"timetable-v{int(m.group(1))+1}", w, count=1)
    open('sw.js', 'w', encoding='utf-8').write(w); print('sw.js: alive.js cached, version bumped')
else: print('sw.js: already hooked')
