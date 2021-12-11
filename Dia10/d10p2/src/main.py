with open('../../input.txt') as f:
	lines =  f.readlines()
	scores = []
	for line in lines:
		stack = []
		for c in line.strip():
			if c in '([{<':
				stack.append(c)
			else:
				x = stack.pop()
				if (x == '(' and c != ')') or (x != '(' and ord(c) - ord(x) != 2):
					break
		else:
			count = 0
			for c in reversed(stack):
				count = count * 5 + '([{<'.index(c) + 1
			scores.append(count)
	scores.sort()
	# print(scores)
	print(scores[int(len(scores) / 2)])


