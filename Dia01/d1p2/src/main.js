readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n').map((v, i, arr) => parseInt(v.trim()));
let sum = 0;
for (let i = 3; i < data.length; i++) {
	if (data[i - 3]+data[i - 2]+data[i - 1] < data[i - 2]+data[i - 1]+data[i])
		sum++;
}

console.log(sum);