/*
 *const fs = require("fs");
 *
 *export function read_dir(path) {
 *  return fs.readdirSync(path);
 *}
 *
 *export function read_file(path) {
 *  return fs.readFileSync(path);
 *}
 *
 */

export async function fetch_dir() {
	var base = window.location;
	let url = base + "data/.index";
	let fut =  await fetch(url); 
	let content = await fut.text();
	let c2 = content.split("\n").filter( n => n);
	console.log(c2);
	return c2
}

export async function fetch_file(path) {
	var base = window.location;
	let url = base + path;
	let fut =  await fetch(url); 
	let content = await fut.text();
	console.log("fetch file", content);
}
