readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n')//.map((v, i, arr) => v.map((v, i, arr) => parseInt(v.trim()));
let d = 0;
let f = 0;
for (let line of data) {
	let s = line.split(' ');

	if(s[0] === 'up') {
		d -= parseInt(s[1])
	}else if(s[0] === 'down') {
		d += parseInt(s[1])
	}else{
		f += parseInt(s[1])
	}
}

console.log(d*f);