with open('../../input.txt') as f:
	lines = list(map(lambda x: list(map(int, list(x.strip()))),  f.readlines()))
	risk = 0
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
				risk += 1 + height
	# row = 0
	# column = 1
	# all_higher = True
	# height = lines[0][1]
	# for k in (-1,1):
	# 	for l in (-1,1):
	# 		if row+k >= 0 and row+k < len(lines) and column+l >= 0 and column+l < len(lines[row]):
	# 			all_higher &= lines[row+k][column+l] > height
	# print(height, all_higher)
	print(risk)


	
