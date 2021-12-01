with open('../../input.txt') as f:
	lines =  f.readlines()
	sum = 0
	for i in range(1, len(lines)):
		if int(lines[i-1]) < int(lines[i]):
			sum += 1
	print(sum)
