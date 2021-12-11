readFileSync = require('fs').readFileSync;

let data = readFileSync('../../input.txt', 'utf8').split('\n')//.map((v, i, arr) => parseInt(v.trim()));
let count = 0;
for(let line of data) {
	stack = []
	// console.log('"'+line+'"')
	for (let c of line.trim()) {
		if (c !== undefined) {
			if ('([{<'.includes(c)) {
				stack.push(c)
			} else {
				x = stack.pop()
				if ((x == '(' && c != ')') || (x != '(' && c.charCodeAt(0) - x.charCodeAt(0) != 2)) {
					if (c == ')') {
						count += 3
					} else if (c == ']') {
						count += 57
					} else if (c == '}') {
						count += 1197
					} else if (c == '>') {
						count += 25137
					} break
				}
			}
		}
		
	}
}

console.log(count);