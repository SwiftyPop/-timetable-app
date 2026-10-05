const V='timetable-v6';
const FILES=[
  './',
  'index.html',
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
