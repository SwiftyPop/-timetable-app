const V='timetable-v10';
const FILES=[
  './',
  'index.html',
  'print.css',
  'qr.svg',
  'backend.js',
  'alive.js',
  'reminders.js',
  'schedule.ics',
  'manifest.webmanifest',
  'icons/icon-192.png',
  'icons/icon-512.png',
  'icons/maskable-512.png',
  'icons/apple-touch-icon.png',
  'fonts/pjs-400.woff2',
  'fonts/pjs-600.woff2',
  'fonts/pjs-800.woff2'
];

self.addEventListener('install', e => {
  e.waitUntil(caches.open(V).then(c => c.addAll(FILES)).then(() => self.skipWaiting()));
});

self.addEventListener('activate', e => {
  e.waitUntil(
    caches.keys().then(k => Promise.all(k.filter(x => x !== V).map(x => caches.delete(x)))).then(() => self.clients.claim())
  );
});

// Network-first for HTML navigation requests (eliminates 2-visit update lag while keeping offline fallback).
// Cache-first for static immutable assets (fonts, icons).
self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  const isNav = e.request.mode === 'navigate' || (e.request.headers.get('accept') && e.request.headers.get('accept').includes('text/html'));
  if (isNav) {
    e.respondWith(
      fetch(e.request)
        .then(res => {
          if (res && res.ok) {
            const cp = res.clone();
            caches.open(V).then(c => c.put(e.request, cp));
          }
          return res;
        })
        .catch(() => caches.match(e.request, { ignoreSearch: true }).then(hit => hit || caches.match('index.html')))
    );
    return;
  }
  e.respondWith(
    caches.match(e.request, { ignoreSearch: true }).then(hit => {
      if (hit) return hit;
      return fetch(e.request).then(res => {
        if (res && res.ok) {
          const cp = res.clone();
          caches.open(V).then(c => c.put(e.request, cp));
        }
        return res;
      });
    })
  );
});

// Tapping a class reminder focuses the app (or opens it).
self.addEventListener('notificationclick', e => {
  e.notification.close();
  const url = (e.notification.data && e.notification.data.url) || './';
  e.waitUntil(self.clients.matchAll({type: 'window', includeUncontrolled: true}).then(cs => {
    for (const c of cs) { if ('focus' in c) { if (c.navigate) c.navigate(url).catch(() => {}); return c.focus(); } }
    return self.clients.openWindow(url);
  }));
});
