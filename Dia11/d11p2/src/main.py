with open('../../input.txt') as f:
	board = list(map(lambda x: list(map(lambda x: [int(x), False], x.strip())), f.readlines()))
	# print(board)
	total = 0
	while(True):
		for y in range(len(board)):
			for x in range(len(board[y])):
				board[y][x][0] += 1

		has_flashed = True
		while has_flashed:
			has_flashed = False
			for y in range(len(board)):
				for x in range(len(board[y])):
					if board[y][x][0] > 9 and not board[y][x][1]:
						has_flashed = True
						board[y][x][1] = True
						for x_new in range(x-1, x+2):
							for y_new in range(y-1, y+2):
								if x_new >= 0 and x_new < len(board[y]) and y_new >= 0 and y_new < len(board):
									board[y_new][x_new][0] += 1
						
				
		
		for y in range(len(board)):
			for x in range(len(board[y])):
				if board[y][x][1]:
					board[y][x] = [0, False]
					
		total += 1
		if all(map(lambda x: all(map(lambda x: x[0] == 0, x)), board)):
			break
	print(total)