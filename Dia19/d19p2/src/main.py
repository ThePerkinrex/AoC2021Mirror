# def rotate(x, y):
# 	return (-y,x)

# def rotations(p):
# 	# print(p)
# 	(x,y,z) = p
# 	(x1,y1) = (x,y)
# 	for i in range(4):
# 		x1,y1 = rotate(x1,y1)
# 		yield (x1,y1,z, i*2)
# 		yield (x1,y1,-z, i*2 + 1)
# 	(x1,z1) = (x,z)
# 	for i in range(4):
# 		x1,z1 = rotate(x1,z1)
# 		yield (x1,y,z1, 8 + i*2)
# 		yield (x1,-y,z1, 9 + i*2)
# 	(y1, z1) = (y,z)
# 	for i in range(4):
# 		y1,z1 = rotate(y1,z1)
# 		yield (x,y1,z1, 16 + i*2)
# 		yield (-x,y1,z1, 17 + i*2)

# def single_indexed_rotation(p, idx):
# 	# print(p)
# 	(x,y,z) = p
# 	if idx < 8:
# 		(x1,y1) = (x,y)
# 		for i in range(int(idx/2) + 1):
# 			x1,y1 = rotate(x1,y1)
# 		if idx%2 == 1:
# 			return (x1,y1,-z)
# 		else:
# 			return (x1,y1,z)
# 	elif idx < 16:
# 		(x1,z1) = (x,z)
# 		for i in range(int((idx-8)/2) + 1):
# 			x1,z1 = rotate(x1,z1)
# 		if idx%2 == 1:
# 			return (x1,-y,z1)
# 		else:
# 			return (x1,y,z1)
# 	else:
# 		(y1, z1) = (y,z)
# 		for i in range(int((idx-16)/2) + 1):
# 			y1,z1 = rotate(y1,z1)
# 		if idx%2 == 1:
# 			return (-x,y1,z1)
# 		else:
# 			return (x,y1,z1)

def rotate_scanner(scanner):

    rotations_axis = [
        lambda a: (a[0], a[1], a[2]),
        lambda a: (a[0], a[2], a[1]),
        lambda a: (a[1], a[0], a[2]),
        lambda a: (a[1], a[2], a[0]),
        lambda a: (a[2], a[0], a[1]),
        lambda a: (a[2], a[1], a[0]),
    ]
    rotations_rotate = [
        lambda a: (a[0], a[1], a[2]),
        lambda a: (-a[0], -a[1], a[2]),
        lambda a: (a[0], -a[1], -a[2]),
        lambda a: (-a[0], a[1], -a[2]),
        lambda a: (-a[0], -a[1], -a[2]),
        lambda a: (-a[0], a[1], a[2]),
        lambda a: (a[0], -a[1], a[2]),
        lambda a: (a[0], a[1], -a[2]),
    ]

    rotations = []
    for axis_rotation in rotations_axis:
        for rotate_rotation in rotations_rotate:
            rotated = list(map(lambda point: axis_rotation(rotate_rotation(point)), scanner))
            rotations.append(rotated)
    return rotations

def move_scanner(scanner, diff):
	return list(map(lambda x: (diff[0]+x[0],diff[1]+x[1],diff[2]+x[2]), scanner))



# def rel_distances(scanner):
# 	res = [[None for _ in scanner] for _ in scanner]
# 	for (i, (x1, y1, z1)) in enumerate(scanner):
# 		# print(v)
# 		# (x1, y1, z1) = v
# 		for (j, (x2, y2, z2)) in enumerate(scanner):
# 			if i != j:
# 				diff = (x2-x1, y2-y1, z2-z1)
# 				# if diff[0] < 0 or (diff[0] == 0 and diff[1] < 0) or (diff[0] == 0 and diff[1] == 0 and diff[2] < 0):
# 				# 	diff = (-diff[0], -diff[1], -diff[2])
# 				res[i][j] = diff
# 	return res

def matches(scanner1, scanner2):
	relations = {}
	for (i, r2) in enumerate(scanner2):
		for (j, r1) in enumerate(scanner1):
			diff = (r1[0] - r2[0], r1[1] - r2[1], r1[2] - r2[2])
			if diff in relations:
				relations[diff].append(r1)
			else:
				relations[diff] = [r1]
	# for v in relations:
	# 	print(len(v))
	matching = list(filter(lambda x: len(x[1]) >= 12, zip(relations.keys(), relations.values())))
	
	if len(matching) == 0:
		return None
	# print(matching[0][0])
	return matching[0][0]

def try_merge(scanners):
	for (i, scanner1) in enumerate(scanners):
		for (j, scanner2) in enumerate(scanners):
			if j > i:
				for scanner2 in rotate_scanner(scanner2):
					matching = matches(scanner1, scanner2)
					if matching is not None:
						moved_scanner = move_scanner(scanner2, matching)
						for p in scanner1:
							if p not in moved_scanner:
								moved_scanner.append(p)
						return (i,j,matching, moved_scanner)


with open('../../input.txt') as f:
	scanners = []
	for scanner in f.read().split('\n\n'):
		lines = iter(scanner.splitlines())
		next(lines)
		scanners.append(list(map(lambda line: tuple(map(int, line.split(','))), lines)))
	
	distances = []
	while len(scanners) > 1:
		matching = try_merge(scanners)
		if matching is None:
			print('ERROR')
			exit(1)
		(i,j,diff,moved_scanner) = matching
		distances.append(diff)
		del scanners[j]
		scanners[i] = moved_scanner
	
	assert len(scanners) == 1

	distance_max = 0
	for (i, p1) in enumerate(distances):
		for (j, p2) in enumerate(distances):
			if j > i:
				distance = abs(p1[0] - p2[0]) + abs(p1[1] - p2[1]) + abs(p1[2] - p2[2])
				if distance > distance_max:
					distance_max = distance
	# for b in sorted(scanners[0], key=lambda x: x[0]):
	# 	print(','.join(map(str, b)))
	print(distance_max)





	

