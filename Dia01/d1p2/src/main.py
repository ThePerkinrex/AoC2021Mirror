with open('../../input.txt') as f:
	lines =  f.readlines()
	sum = 0
	for i in range(3, len(lines)):
		if int(lines[i-3]) + int(lines[i-2]) + int(lines[i-1]) < int(lines[i-2]) + int(lines[i-1]) + int(lines[i]):
			sum += 1
	print(sum)
