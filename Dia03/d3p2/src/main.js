readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n').map(x => x.trim())
let gamma = (new Array(data[0].length)).fill('0')
let epsilon = (new Array(gamma.length)).fill('1')
let counts = []
let count = 0

for (let i = 0; i < gamma.length; i++) {
	for (line of data) {
		if (line[i] == '1') {
			count += 1
			if (count >= data.length / 2) {
				gamma[i] = '1'
				epsilon[i] = '0'
			}
		}
		// console.log(i, line, count)

	}
	counts.push(count)
	// console.log(gamma)
	count = 0
}
// console.log(counts)

let co2 = [...data]
let co2_counts = [...counts]
let o2 = [...data]
let o2_counts = [...counts]

for (let i = 0; i <= gamma.length; i++) {
	let currlen = o2.length
	let currcounts = [...o2_counts]
	for (let j = o2.length-1; j >= 0 && o2.length > 1; j--) {

		if (currcounts[i] >= currlen / 2 && o2[j][i] == '0') {
			for (let x = 0; x < o2[j].length; x++) {
				if (o2[j][x] == '1') {
					o2_counts[x] -= 1
				}
			}
			o2.splice(j,1)
		}else if(currcounts[i] < currlen / 2 && o2[j][i] == '1') {
			for (let x = 0; x < o2[j].length; x++) {
				if (o2[j][x] == '1') {
					o2_counts[x] -= 1
				}
			}
			o2.splice(j,1)
		}
	}
	currlen = co2.length
	currcounts = [...co2_counts]
	for (let j = co2.length-1; j >= 0 && co2.length > 1; j--) {

		if (currcounts[i] >= currlen / 2 && co2[j][i] == '1') {
			for (let x = 0; x < co2[j].length; x++) {
				if (co2[j][x] == '1') {
					co2_counts[x] -= 1
				}
			}
			co2.splice(j,1)
		}else if(currcounts[i] < currlen / 2 && co2[j][i] == '0') {
			for (let x = 0; x < co2[j].length; x++) {
				if (co2[j][x] == '1') {
					co2_counts[x] -= 1
				}
			}
			co2.splice(j,1)
		}
	}
}
console.log(' o2:', o2[0])
console.log('co2:', co2[0])
console.log(' o2:', parseInt(o2[0],2))
console.log('co2:', parseInt(co2[0],2))
console.log('>>>', parseInt(o2[0],2) * parseInt(co2[0],2))