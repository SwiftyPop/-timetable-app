My Timetable — offline web app (PWA)

Quick use (no hosting): open index.html in a desktop browser. Everything is local, no internet needed.

Install as an app on phone AND desktop (needs https hosting, free options):
 1. Upload this whole folder to GitHub Pages, Netlify (drag & drop), or Cloudflare Pages.
 2. Open the link once while online.
 3. Chrome/Edge (desktop or Android): click "Install app" in the page or the install icon in the address bar.
    iPhone/iPad Safari: Share > Add to Home Screen.
 After that it opens offline like a normal app.

To change classes later, edit the list "var E=[...]" in index.html.
Regenerate calendar: python tools/make_ics.py
Then rebuild dist: powershell -File scripts/optimize.ps1
Bump cache version in sw.js (e.g. timetable-v8 -> timetable-v9).

New Features:
- Class Reminders & Google Calendar Sync: see SETUP.md
- Alive Motion & Ambient Layer (alive.js): console helpers alive.celebrate(), alive.spark(x,y)

