if(!self.define){const e=e=>{"require"!==e&&(e+=".js");let s=Promise.resolve();return r[e]||(s=new Promise((async s=>{if("document"in self){const r=document.createElement("script");r.src=e,document.head.appendChild(r),r.onload=s}else importScripts(e),s()}))),s.then((()=>{if(!r[e])throw new Error(`Module ${e} didn’t register its module`);return r[e]}))},s=(s,r)=>{Promise.all(s.map(e)).then((e=>r(1===e.length?e[0]:e)))},r={require:Promise.resolve(s)};self.define=(s,i,n)=>{r[s]||(r[s]=Promise.resolve().then((()=>{let r={};const t={uri:location.origin+s.slice(1)};return Promise.all(i.map((s=>{switch(s){case"exports":return r;case"module":return t;default:return e(s)}}))).then((e=>{const s=n(...e);return r.default||(r.default=s),r}))})))}}define("./service-worker.js",["./workbox-08e0b74e"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"/d8f1e7d1ee54bb184895.module.wasm",revision:null},{url:"/index.html",revision:"15ed0b32879b8f4162340e13d9182cb5"},{url:"/static/css/main.1a50e72a.chunk.css",revision:null},{url:"/static/js/0.93411c02.chunk.js",revision:null},{url:"/static/js/3.5b5406d8.chunk.js",revision:null},{url:"/static/js/3.5b5406d8.chunk.js.LICENSE.txt",revision:"6947649729233a9261c1628967141b84"},{url:"/static/js/4.9a47574f.chunk.js",revision:null},{url:"/static/js/5.ba7c54f9.chunk.js",revision:null},{url:"/static/js/main.d06216cf.chunk.js",revision:null},{url:"/static/js/runtime-main.ec4b445f.js",revision:null}],{})}));
//# sourceMappingURL=service-worker.js.map
