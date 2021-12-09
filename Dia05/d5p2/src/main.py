with open('../../input.txt') as f:
	lines = list(map(lambda x: tuple(map(lambda x: tuple(map(int, x.split(','))), x.split(" -> "))), f.readlines()))

	max_hor = 0
	max_ver = 0
	for ((x1, y1), (x2, y2)) in lines:
		# print((x1,y1), (x2,y2))
		if x1 > max_hor:
			max_hor = x1
		if x2 > max_hor:
			max_hor = x2
		if y1 > max_ver:
			max_ver = y1
		if y2 > max_ver:
			max_ver = y2
	print(max_hor, max_ver)
	max_hor += 1
	max_ver += 1
	
	m = [[0 for _ in range(max_ver)] for _ in range(max_hor)]
	for ((x1, y1), (x2, y2)) in lines:
		
		if x1 == x2:
			# print((x1,y1), (x2,y2))
			for y in range(abs(y1-y2)+1):
				m[x1][min(y1,y2) + y] += 1
		elif y1 == y2:
			# print((x1,y1), (x2,y2))
			for x in range(abs(x1-x2)+1):
				# print(m[x][y1])
				m[min(x1, x2) + x][y1] += 1
		else: # diagonal
			xdir = x2 - x1
			ydir = y2 - y1
			print((x1,y1), (x2,y2))
			print(xdir, ydir)
			for t in range(abs(xdir) + 1):
				print(x1 + int(xdir / abs(xdir)) * t, y1 + int(ydir / abs(ydir)) * t, end=', ')
				m[x1 + int(xdir / abs(xdir)) * t][y1 + int(ydir / abs(ydir)) * t] += 1
			print("")

	# print(m)
	count = 0
	for y in range(len(m[0])):
		for x in range(len(m)):
			print('.' if m[x][y] == 0 else str(m[x][y]), end='')
			if m[x][y] > 1:
				count+=1
		print()
	print(count)