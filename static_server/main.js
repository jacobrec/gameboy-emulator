/**
 * This is the main web server, it just serves static content over https
 */
const fs = require('fs');
const https = require('https');
const express = require('express');

const keypath = '/etc/letsencrypt/live/reckhard.ca/';
const app = express();
app.use(express.static(process.env.SERVE_DIRECTORY || 'public'));
app.get('/', function(req, res) {
  return res.sendFile('./public/index.html', { root: __dirname });
});

const options = {
   key: fs.readFileSync(keypath+'privkey.pem', 'utf8'),
  cert: fs.readFileSync(keypath+'fullchain.pem', 'utf8'),
  passphrase: process.env.HTTPS_PASSPHRASE || ''
};
const server = https.createServer(options, app);

server.listen(process.env.SERVER_PORT || 8443);

