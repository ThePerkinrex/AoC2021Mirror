import colorama
from colorama import Fore

def print_board(b):
	for i in range(len(b)):
		print(">>> ", end='')
		for j in range(len(b[i])):
			print((Fore.RED if b[i][j][1] else '') +  str(b[i][j][0]) + Fore.RESET +' ', end='')
		print()		
					


with open('../../input2.txt') as f:
	draw_nums = map(int, f.readline().split(','))
	f.readline()
	boards = list(
		map(
			lambda x: (
				list(
					map(
						lambda x: list(
							map(
								lambda x: (int(x), False), x.split()
							)
						),
						x.splitlines()
					)
				),
				True
				),
			f.read().split('\n\n')
		)
	)
	# print(boards[0].flatten())
	winner = None
	last_num = None
	count = len(boards)
	for num in draw_nums:
		if winner is not None:
			break
		
		# last_num = num
		for i in range(len(boards)):

			if count == 0:
				break

			# print("###############", i, "###############")
			# print_board(boards[i][0])
			if boards[i][1]:
				for j in range(len(boards[i][0])):
					
					if count == 0:
						break
					for k in range(len(boards[i][0][j])):
						# Update
						if boards[i][0][j][k][0] == num:
							# print(boards[i][0][j][k][0], num)
							boards[i][0][j][k] = (num, True)
							break
					else:
						continue
					break
				for j in range(len(boards[i][0])):
					row = True
					column = True
					for k in range(len(boards[i][0][j])):
						row &= boards[i][0][j][k][1]
						column &= boards[i][0][k][j][1]
						# if i == 34 and num == 81:
							# print(j,k, row, column)
						
					if (row or column )and boards[i][1]:
						print(i, 'Row:', row, '     Column:', column, "      Last:", num, "  > COUNT:", count)
						b = boards[i][0]
						boards[i] = (b, False)
						count -= 1
						# print("Board", i, "is a winner")
						
						# print_board(boards[i][0])
						# print()
						# print_board(boards[34][0])
						if count == 0:
							winner = i
			
		last_num = num
			# else:
	# for (i, (b, x)) in enumerate(boards):
	# 	if x:
	# 		winner = i
	unmarked = 0
	for i in range(len(boards[winner][0])):
		for j in range(len(boards[winner][0][i])):
			if not boards[winner][0][i][j][1]:
				unmarked += boards[winner][0][i][j][0]	
	# print(winner)
	# print_board(boards[winner][0])
	print("Unmarked:", unmarked)
	print("Last:", last_num)
	print(last_num * unmarked)
			

		