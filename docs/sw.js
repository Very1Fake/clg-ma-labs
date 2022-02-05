var cacheName = 'clg-ma-lab-6';
var filesToCache = [
  './',
  './index.html',
  './lab_6.js',
  './lab_6_bg.wasm',
];

self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
