def path(node, connections, visited):
	if node == "end":
		return [["end"]]
	res = []
	visited = visited.copy()
	visited.append(node)
	for c in connections[node]:
		if c.isupper() or c not in visited:
			# print(node, ">", c)
			for x in path(c, connections, visited):
				x.insert(0, node)
				res.append(x)
	return res



with open('../../input.txt') as f:
	connections = {}
	for [n0, n1] in map(lambda x: x.strip().split('-'), f.readlines()):
		if n0 not in connections:
			connections[n0] = []
		if n1 not in connections:
			connections[n1] = []
		connections[n0].append(n1)
		connections[n1].append(n0)
	# print(connections["start"])
	print(len(path("start", connections, [])))