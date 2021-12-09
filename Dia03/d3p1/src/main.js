readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n')
let gamma = (new Array(data[0].length)).fill('0')
let epsilon = (new Array(gamma.length)).fill('1')
count = 0


for (let i = 0; i < gamma.length-1; i++) {
	for (line of data) {
		if (line[i] == '1') {
			count += 1
			if (count >= data.length / 2) {
				gamma[i] = '1'
				epsilon[i] = '0'
				break
			}
		}
	}
	// console.log(gamma)
	count = 0
}
gamma = (parseInt(gamma.join(''), 2) >> 1)
epsilon = (parseInt(epsilon.join(''), 2) >> 1)
console.log("gamma: " + gamma)
console.log("epsilon: " + epsilon)
console.log('>>> ' + epsilon * gamma)