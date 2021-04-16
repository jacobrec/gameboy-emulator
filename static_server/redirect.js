/**
 * This just redirects http requests on port 80 to https requests on port 443
 */
var express = require('express')
var app = express();

app.get('*', function(req, res) {
  return res.redirect('https://' + req.headers.host + req.url);
})

app.listen(process.env.PORT || 80);
