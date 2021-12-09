with open('../../input.txt') as f:
	lines = list(map(int, f.readline().split(',')))
	lines.sort()
	avg = sum(lines) / len(lines)
	cheapest = 10000000000000000000000000000000000000000000000000000
	cheapest_pos = None
	for x in range(lines[0], lines[-1]):
		cost = sum(map(lambda v: abs(x-v), lines))
		if cost < cheapest:
			cheapest = cost
			cheapest_pos = x
	print(cheapest, cheapest_pos)
	
