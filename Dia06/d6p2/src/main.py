def print_status(x):
	for i in range(len(x)):
		for _ in range(x[i]):
			print(i, end=',')
	print()

with open('../../input.txt') as f:
	lines = list(map(int, f.readline().split(',')))
	fish = [0 for x in range(9)]
	for x in lines:
		fish[x+1] += 1
	for x in range(257):
		# print(x/0.8)
		a = fish[0]
		for i in range(8):
			fish[i] = fish[i+1]
		fish[6] += a
		fish[8] = a
		# print(x, end=":\t")
		# print_status(fish)
	count = 0
	for x in fish:
		count += x
	print(">>>", count)
	
