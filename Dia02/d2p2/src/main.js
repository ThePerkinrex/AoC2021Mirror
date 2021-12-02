readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n')//.map((v, i, arr) => v.map((v, i, arr) => parseInt(v.trim()));
let d = 0;
let f = 0;
let aim = 0;
for (let line of data) {
	let s = line.split(' ');

	if(s[0] === 'up') {
		aim -= parseInt(s[1])
	}else if(s[0] === 'down') {
		aim += parseInt(s[1])
	}else{
		let x = parseInt(s[1])
		f += x
		d += x * aim
	}
}

console.log(d*f);