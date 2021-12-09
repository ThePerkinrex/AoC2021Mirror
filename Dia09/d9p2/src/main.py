def higher_points(row, column, lines, closed):
	height = lines[row][column]
	# changed = False
	for k in (-1,1):
		if row+k >= 0 and row+k < len(lines) and lines[row+k][column] > height and lines[row+k][column] != 9 and (row+k, column) not in closed:
			higher_points(row+k, column, lines, closed)
			closed.append((row+k, column))
		if column+k >= 0 and column+k < len(lines[row]) and lines[row][column+k] > height and lines[row][column+k] != 9 and (row, column+k) not in closed:
			higher_points(row, column+k, lines, closed)
			closed.append((row, column+k))

with open('../../input.txt') as f:
	lines = list(map(lambda x: list(map(int, list(x.strip()))),  f.readlines()))
	low_points = []
	for row in range(len(lines)):
		for column in range(len(lines[row])):
			# print(row, column)
			height = lines[row][column]
			all_higher = True
			for k in (-1,1):
				if row+k >= 0 and row+k < len(lines):
					# print(">>", row+k, column, row+k >= 0 and row+k < len(lines))
					all_higher &= lines[row+k][column] > height
				if column+k >= 0 and column+k < len(lines[row]):
					# print(">>", row, column+k, column+k >= 0 and column+k < len(lines[row]))
					all_higher &= lines[row][column+k] > height
			if all_higher:
				# print("HERE")
				closed = []
				higher_points(row, column, lines, closed)
				# print(closed)
				low_points.append(len(closed) + 1)
	low_points.sort()
	print(low_points[-3] * low_points[-2] * low_points[-1] )
	
	

	


	
