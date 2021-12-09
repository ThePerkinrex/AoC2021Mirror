with open('../../input.txt') as f:
	count = 0
	for line in f.readlines():
		digits = line.split(' | ')[1].split()
		for d in digits:
			if len(d) in (2,3,4,7):
				count+=1
	print(count)
