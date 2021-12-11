
def flash(board, x, y):
	print("FLASH", x, y, f'[{board[y][x]}]', end=' (')
	flashes = 0
	board[y][x] += 1
	for x_diff in range(-1, 2):
		for y_diff in range(-1, 2):
			if x_diff != 0 or y_diff != 0 and (x+x_diff >= 0 and x+x_diff < len(board[y])) and  (y+y_diff >= 0 and y+y_diff < len(board)):
				print(f'<{x+x_diff}, {y+y_diff}>', end='')
				board[y+y_diff][x+x_diff] += 1
				if board[y+y_diff][x+x_diff] == 10:
					print(" -> ", end='')
					flashes += flash(board, x+x_diff, y+y_diff) + 1
	print(')', end='')
	return flashes

with open('../../input.txt') as f:
	board = list(map(lambda x: list(map(lambda x: [int(x), False], x.strip())), f.readlines()))
	# print(board)
	total = 0
	for _ in range(100):
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
	print(total)