with open('../../input.txt') as f:
	lines =  f.readlines()
	count = 0
	for line in lines:
		stack = []
		for c in line.strip():
			if c in '([{<':
				stack.append(c)
			else:
				x = stack.pop()
				if (x == '(' and c != ')') or (x != '(' and ord(c) - ord(x) != 2):
					if c == ')':
						count += 3
					elif c == ']':
						count += 57
					elif c == '}':
						count += 1197
					elif c == '>':
						count += 25137
					break
	print(count)


