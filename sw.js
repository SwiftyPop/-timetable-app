const V='timetable-v1';
const FILES=["./", "index.html", "manifest.webmanifest", "icons/icon-192.png", "icons/icon-512.png", "icons/maskable-512.png", "icons/apple-touch-icon.png", "fonts/pf-500.woff2", "fonts/pf-500i.woff2", "fonts/pjs-400.woff2", "fonts/pjs-500.woff2", "fonts/pjs-600.woff2", "fonts/pjs-700.woff2", "fonts/pjs-800.woff2"];
self.addEventListener('install',e=>{e.waitUntil(caches.open(V).then(c=>c.addAll(FILES)).then(()=>self.skipWaiting()))});
self.addEventListener('activate',e=>{e.waitUntil(caches.keys().then(k=>Promise.all(k.filter(x=>x!==V).map(x=>caches.delete(x)))).then(()=>self.clients.claim()))});
// Cache first so it opens instantly offline; refresh the copy in the background when online.
self.addEventListener('fetch',e=>{
  if(e.request.method!=='GET')return;
  e.respondWith(caches.match(e.request,{ignoreSearch:true}).then(hit=>{
    const net=fetch(e.request).then(res=>{if(res&&res.ok){const cp=res.clone();caches.open(V).then(c=>c.put(e.request,cp))}return res}).catch(()=>hit||caches.match('index.html'));
    return hit||net;
  }));
});
