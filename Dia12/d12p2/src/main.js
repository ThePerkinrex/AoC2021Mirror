function path(node, connections, visited, double) {
	if (node === "end") {
		return [["end"]]
	}
	let res = []
	visited = [node, ...visited]
	for (let c of connections[node]) {
		if (!visited.includes(c) || c === c.toUpperCase()) {
			for (let x of path(c, connections, visited, double)) {
				res.push([node, ...x])
			}
		}else if(double && c !== "start") {
			for (let x of path(c, connections, visited, false)) {
				res.push([node, ...x])
			}
		}
	}
	// console.log(res.length, res[0])
	return res
}

readFileSync = require('fs').readFileSync;
connections = {}
for(let line of readFileSync('../../input.txt', 'utf8').split('\n').map(v => v.trim().split('-'))) {
	if (connections[line[0]] === undefined) {
		connections[line[0]] = []
	}
	if (connections[line[1]] === undefined) {
		connections[line[1]] = []
	}
	connections[line[0]].push(line[1])
	connections[line[1]].push(line[0])
}

console.log(path("start", connections, [], true).length)