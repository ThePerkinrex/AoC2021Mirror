def print_grid(rows, cols, hor, ver):
	for i in range(rows):
		for j in range(cols):
			if (i,j) in hor:
				print('>', end = '')
			elif (i,j) in ver:
				print('v', end = '')
			else:
				print('.', end = '')
		print()
			

with open('../../input.txt') as f:
	grid = list(map(lambda x: list(x.strip()), f.readlines()))
	rows = len(grid)
	cols = len(grid[0])
	hor_mov = {}
	ver_mov = {}
	for (i, row) in enumerate(grid):
		for (j, v) in enumerate(row):
			if v == 'v':
				ver_mov[(i,j)] = None
			elif v == '>':
				hor_mov[(i,j)] = None
	count = 0
	updated = True
	while updated:
		# print(count)
		# print_grid(rows, cols, hor_mov, ver_mov)
		updated = False
		hor = list(hor_mov.keys())
		ver = list(ver_mov.keys())
		for (i,j) in hor:
			new_pos = (i,(j + 1) % cols)
			if new_pos not in hor and new_pos not in ver:
				del hor_mov[(i,j)]
				hor_mov[new_pos] = None
				updated = True
		
		hor = list(hor_mov.keys())
		for (i,j) in ver:
			new_pos = ((i+1) % rows, j)
			if new_pos not in hor and new_pos not in ver:
				del ver_mov[(i,j)]
				ver_mov[new_pos] = None
				updated = True
		count += 1
		# if count > 3 :
		# 	break
	print(count)
