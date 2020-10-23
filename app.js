#!/bin/node

var express = require('express')
var bodyParser = require('body-parser');
var app = express();

app.use(bodyParser.json());         // to support JSON-encoded bodies
app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
  extended: true
}));

app.use(express.static('public'));

app.listen(3000, function () {
  console.log('server was started on http://localhost:3000');
});
