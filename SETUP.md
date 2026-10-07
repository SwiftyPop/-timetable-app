# Notifications + Google Calendar — setup

## 1. Add the files to your repo
Copy these into the repo root (replace the two existing ones):
- `index.html`   (replaced: adds `<script src="reminders.js" defer>`, fixes the .ics description line breaks)
- `sw.js`        (replaced: cache v7, caches the new files, adds a notification-click handler)
- `reminders.js` (new)
- `schedule.ics` (new — the file Google Calendar subscribes to)
- `tools/make_ics.py` (new, optional — regenerates `schedule.ics`)

Commit and push. Open the site once online so the service worker updates.

## 2. Class reminders (no setup)
Settings → Class reminders → **Enable** → allow the permission. Pick 5/10/15/30 min, optional 07:30 morning summary, mute subjects.
- Works while the app is open or running in the background (Android Chrome, desktop).
- iPhone/iPad: only after "Add to Home Screen" (iOS 16.4+), and the permission must be granted from the button tap.
- A phone may pause a background web page, so for guaranteed pop-ups use Google Calendar Sync below.

## 3. Google Calendar — two options
**Subscribe (no setup):** Settings → Google Calendar → ➕ Subscribe. Read-only, refreshes itself (Google can take several hours). Set the pop-up time under the calendar's own notification settings in Google Calendar, because subscribed calendars ignore reminders inside the file.

**Sync (editable events + phone pop-ups):** needs a free Google OAuth client ID, once:
1. https://console.cloud.google.com → create a project → **APIs & Services → Library** → enable **Google Calendar API**.
2. **OAuth consent screen** → External → fill the basics → add scope `.../auth/calendar.app.created` → under **Test users** add your own Google account.
3. **Credentials → Create credentials → OAuth client ID → Web application**. Under **Authorized JavaScript origins** add `https://swiftypop.github.io` (origin only, no path).
4. Copy the client ID into Settings → "Google OAuth client ID", then press **🔄 Sync**.

Sync creates a separate calendar called "UniMAP Timetable" with weekly repeating events and a pop-up reminder (your chosen minutes). Press Sync again after any schedule or lead-time change; it updates existing events and removes ones that no longer exist. **🗑 Remove** deletes that calendar. The app only has access to calendars it creates itself.

## Keeping things in step when the schedule changes
1. Edit the `E` array in `index.html`.
2. Run `python3 tools/make_ics.py` to regenerate `schedule.ics`.
3. Bump `timetable-v7` in `sw.js`, then commit.

Semester end (`20270228T160000Z`) lives in `reminders.js`, `tools/make_ics.py` and the existing `buildVEvent` in `index.html`.
