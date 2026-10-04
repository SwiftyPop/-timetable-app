My Timetable — offline web app (PWA)

Quick use (no hosting): open index.html in a desktop browser. Everything is local, no internet needed.

Install as an app on phone AND desktop (needs https hosting, free options):
 1. Upload this whole folder to GitHub Pages, Netlify (drag & drop), or Cloudflare Pages.
 2. Open the link once while online.
 3. Chrome/Edge (desktop or Android): click "Install app" in the page or the install icon in the address bar.
    iPhone/iPad Safari: Share > Add to Home Screen.
 After that it opens offline like a normal app.

To change classes later, edit the list "var E=[...]" in index.html. If you update files, bump the version in sw.js (timetable-v1 -> v2).
