//import  "./showdown.js";
//const showdown = require("showdown");
importScripts("./showdown.js")

converter = new showdown.Converter();
text = "# hello showdown!";
html = converter.makeHtml(text);
console.log(html);
