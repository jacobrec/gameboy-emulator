var CACHE_NAME = 'pwa-gameboy-emulator-cache';
var urlsToCache = [
  '/',
  '/index.html',
  '/static/js/bundle.js',
  '/static/js/0.chunk.js',
  '/static/js/1.chunk.js',
  '/static/js/2.chunk.js',
  '/static/js/main.chunk.js',
  '/favicon.ico',
  '/logo192.png',
  '/logo512.png',
  '/e6c918b637992db06180.module.wasm',
];
  'static/js/0.chunk.js': '/static/js/0.chunk.js',
  'module.wasm': '/afaff60a829a10d9d6b6.module.wasm',
  'static/js/0.chunk.js.map': '/static/js/0.chunk.js.map',
  'static/js/1.chunk.js': '/static/js/1.chunk.js',
  'static/js/1.chunk.js.map': '/static/js/1.chunk.js.map',
  'static/js/2.chunk.js': '/static/js/2.chunk.js',
  'static/js/2.chunk.js.map': '/static/js/2.chunk.js.map',
  'static/js/3.chunk.js': '/static/js/3.chunk.js',
  'static/js/3.chunk.js.map': '/static/js/3.chunk.js.map',
  'main.js': '/static/js/main.chunk.js',
  'main.js.map': '/static/js/main.chunk.js.map',
  'runtime-main.js': '/static/js/bundle.js',
  'runtime-main.js.map': '/static/js/bundle.js.map',
  'index.html': '/index.html'

// Install a service worker
self.addEventListener('install', event => {
  // Perform install steps
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) {
        console.log('Opened cache');
        return cache.addAll(urlsToCache);
      })
  );
});

// Cache and return requests
self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(function(response) {
        // Cache hit - return response
        if (response) {
          return response;
        }
        return fetch(event.request);
      }
    )
  );
});

// Update a service worker
self.addEventListener('activate', event => {
  var cacheWhitelist = ['pwa-task-manager'];
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return Promise.all(
        cacheNames.map(cacheName => {
          if (cacheWhitelist.indexOf(cacheName) === -1) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});
