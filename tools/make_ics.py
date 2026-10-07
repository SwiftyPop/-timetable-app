#!/usr/bin/env python3
"""Regenerate schedule.ics from the E / S arrays in index.html.  Usage: python3 tools/make_ics.py [index.html] [schedule.ics]"""
import re, sys, datetime as dt
src = sys.argv[1] if len(sys.argv) > 1 else 'index.html'
out = sys.argv[2] if len(sys.argv) > 2 else 'schedule.ics'
FIRST_MONDAY = dt.date(2026, 10, 5)      # first week shown in the calendar
UNTIL = '20270228T160000Z'               # keep in sync with reminders.js / index.html
html = open(src, encoding='utf-8').read()
S = {m[0]: (m[1], m[2]) for m in re.findall(r"(\w+):\{n:'([^']*)',c:'([^']*)'", html) and
     [(a, b, c) for a, b, c in re.findall(r"(\w+):\{n:'([^']*)',c:'([^']*)'", html)]}
block = re.search(r"var E=\[(.*?)\n\];", html, re.S).group(1)
E = re.findall(r"\[(\d),(\d+),(\d+),'(\w+)','([^']*)','([^']*)','([^']*)'\]", block)
def esc(t): return t.replace('\\', '\\\\').replace(';', '\\;').replace(',', '\\,').replace('\n', '\\n')
def fold(line):
    b = line.encode('utf-8'); parts = []
    while len(b) > 75:
        cut = 75
        while (b[cut] & 0xC0) == 0x80: cut -= 1
        parts.append(b[:cut]); b = b[cut:]
        b = b' ' + b
    parts.append(b)
    return '\r\n'.join(p.decode('utf-8') for p in parts)
stamp = dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
L = ['BEGIN:VCALENDAR', 'VERSION:2.0', 'PRODID:-//UniMAP Timetable App//EN', 'CALSCALE:GREGORIAN', 'METHOD:PUBLISH',
     'X-WR-CALNAME:UniMAP Timetable', 'X-WR-TIMEZONE:Asia/Kuala_Lumpur', 'REFRESH-INTERVAL;VALUE=DURATION:PT12H',
     'BEGIN:VTIMEZONE', 'TZID:Asia/Kuala_Lumpur', 'BEGIN:STANDARD', 'DTSTART:19700101T000000', 'TZOFFSETFROM:+0800',
     'TZOFFSETTO:+0800', 'TZNAME:MYT', 'END:STANDARD', 'END:VTIMEZONE']
for d, s0, e0, key, typ, room, lect in E:
    d, s0, e0 = int(d), int(s0), int(e0)
    day = FIRST_MONDAY + dt.timedelta(days=d - 1)
    name, code = S[key]
    L += ['BEGIN:VEVENT', f'UID:class-{d}-{s0}-{key}-2026@timetable.local', f'DTSTAMP:{stamp}',
          'SUMMARY:' + esc(f'{name} ({typ})'), 'DESCRIPTION:' + esc(f'{code}\nVenue: {room}\nLecturers: {lect or "—"}'),
          'LOCATION:' + esc(room),
          f'DTSTART;TZID=Asia/Kuala_Lumpur:{day:%Y%m%d}T{s0:02d}0000', f'DTEND;TZID=Asia/Kuala_Lumpur:{day:%Y%m%d}T{e0:02d}0000',
          f'RRULE:FREQ=WEEKLY;UNTIL={UNTIL};BYDAY=' + ['MO', 'TU', 'WE', 'TH', 'FR'][d - 1], 'STATUS:CONFIRMED',
          'BEGIN:VALARM', 'TRIGGER:-PT15M', 'ACTION:DISPLAY', 'DESCRIPTION:' + esc(f'{name} starts in 15 minutes'), 'END:VALARM', 'END:VEVENT']
L.append('END:VCALENDAR')
open(out, 'w', encoding='utf-8', newline='').write('\r\n'.join(fold(l) for l in L) + '\r\n')
print(f'wrote {out}: {len(E)} weekly classes')
