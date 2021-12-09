with open('../../input2.txt') as f:
	lines = list(map(int, f.readline().split(',')))
	for x in range(80):
		print(x/0.8)
		for i in range(len(lines)):
			if lines[i] == 0:
				lines[i] = 6
				lines.append(8)
			else:
				lines[i] -= 1
	print(">>>", len(lines))
	
