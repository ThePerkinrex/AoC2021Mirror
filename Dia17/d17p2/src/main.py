def isValidStart(vx, vy, xmin, xmax, ymin, ymax):
	x = 0
	y = 0
	t = 0
	prevx = x
	prevy = y
	maxy = 0
	while x <= xmax and y >= ymin:
		prevx = x
		prevy = y

		x += vx
		y += vy
		if vx > 0:
			vx -= 1
		vy -= 1
		t += 1
		if y > maxy:
			maxy = y
		# print('>>', t, x,y)
	return (prevx >= xmin and prevx <= xmax and prevy >= ymin and prevy <= ymax, maxy)
		

with open('../../input.txt') as f:
	((xmin, xmax), (ymin, ymax)) = tuple(map(lambda x: tuple(map(int, x[2:].split('..'))),f.readline().strip()[13:].split(', ')))
	count = 0
	for vox in range(int((2*xmin)**0.5), xmax+1):
		for voy in range(abs(ymin), ymin-1, -1):
			print('>', vox, voy)
			(valid, height) = isValidStart(vox,voy, xmin, xmax, ymin, ymax)
			if valid:
				print(count, vox, voy)
				count += 1
	print()
	# isValidStart(6,9, xmin, xmax, ymin, ymax)
	print(count)
