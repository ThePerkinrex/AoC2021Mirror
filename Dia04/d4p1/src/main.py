

def print_board(b):
	for i in range(len(b)):
		for j in range(len(b[i])):
			print(str(b[i][j]) + ' ', end='')
		print()		
					


with open('../../input.txt') as f:
	draw_nums = map(int, f.readline().split(','))
	f.readline()
	boards = list(map(lambda x: list(map(lambda x: list(map(lambda x: (int(x), False), x.split())), x.splitlines())),f.read().split('\n\n')))
	# print(boards[0].flatten())
	winner = None
	last_num = None
	for num in draw_nums:
		if winner is not None:
			break
		
		last_num = num
		for i in range(len(boards)):
			if winner is not None:
				break
			for j in range(len(boards[i])):
				if winner is not None:
					break
				row = True
				column = True
				for k in range(len(boards[i][j])):
					# Update
					if boards[i][j][k][0] == num:
						boards[i][j][k] = (num, True)
					
					row &= boards[i][j][k][1]
					column &= boards[i][k][j][1]
				
				if row or column:
					print(i, 'Row:', row, '     Column:', column)
					winner = i
	unmarked = 0
	for i in range(len(boards[winner])):
		for j in range(len(boards[winner][i])):
			if not boards[winner][i][j][1]:
				unmarked += boards[winner][i][j][0]	
	# print_board(boards[winner])
	# print("Unmarked:", unmarked)
	print(last_num * unmarked)
			

		