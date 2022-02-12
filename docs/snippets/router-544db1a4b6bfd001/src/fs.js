const fs = require("fs");

export function read_dir(path) {
	return fs.readdirSync(path);
}

export function read_file(path) {
	return fs.readFileSync(path);
}
