################ OLD CODE

# with open('../../input.txt') as f:

# 	cuboids = list(map(lambda x: (x.split(' ')[0] == 'on', tuple(map(lambda x: tuple(map(int, x[2:].split('..'))), x.split()[1].strip().split(',')))), f.readlines()))

# 	for x in cuboids:
# 		print(x)
# 	on = {}
# 	for (i, (is_on, ((x_min, x_max), (y_min, y_max), (z_min, z_max)))) in enumerate(cuboids):
# 		print(i)
# 		for x in filter(lambda x: x in range(-50,51), range(x_min, x_max+1)):
# 			for y in filter(lambda x: x in range(-50,51), range(y_min, y_max+1)):
# 				for z in filter(lambda x: x in range(-50,51), range(z_min, z_max+1)):
# 					if (x,y,z) in on and not is_on:
# 						del on[(x,y,z)]
# 					elif (x,y,z) not in  on and is_on:
# 						# if x in range(-50, 51) and y in range(-50,51) and z in range(-50, 51):
# 						on[(x,y,z)] = True
# 	print(len(on))


def cube_intersection(a, b):
	((ax_min, ay_min, az_min), (ax_max, ay_max, az_max)) = a
	((bx_min, by_min, bz_min), (bx_max, by_max, bz_max)) = b
	((rx_min, ry_min, rz_min), (rx_max, ry_max, rz_max)) = ((max(ax_min, bx_min), max(ay_min, by_min), max(az_min, bz_min)), (min(ax_max, bx_max), min(ay_max, by_max), min(az_max, bz_max)))

	if rx_min < rx_max and ry_min < ry_max and rz_min < rz_max:
		return ((rx_min, ry_min, rz_min), (rx_max, ry_max, rz_max))


def split_cube(a, i):
	((ax_min, ay_min, az_min), (ax_max, ay_max, az_max)) = a
	((ix_min, iy_min, iz_min), (ix_max, iy_max, iz_max)) = i
	
	res = []
	if ax_min < min(ax_max, ix_min):
		res.append(((ax_min, ay_min, az_min), (min(ax_max, ix_min), ay_max, az_max)))
	if max(ax_min, ix_max) < ax_max:
		res.append(((max(ax_min, ix_max), ay_min, az_min), (ax_max, ay_max, az_max)))
	if ay_min < min(ay_max, iy_min):
		res.append(((ix_min, ay_min, az_min), (ix_max, min(ay_max, iy_min), az_max)))
	if max(ay_min, iy_max) < ay_max:
		res.append(((ix_min, max(ay_min, iy_max), az_min), (ix_max, ay_max, az_max)))
	if az_min < min(az_max, iz_min):
		res.append(((ix_min, iy_min, az_min), (ix_max, iy_max, min(az_max, iz_min))))
	if max(az_min, iz_max) < az_max:
		res.append(((ix_min, iy_min, max(az_min, iz_max)), (ix_max, iy_max, az_max)))
	return res


# def turn_off(og, off_cubes, to_remove):
# 	intersect = [cube_intersection(og, to_remove)]
# 	print('TURN OFF')
# 	if intersect[0] is not None:
# 		updated  = True
# 		while updated:
# 			updated = False
			
# 			for off_cube in off_cubes:
# 				for j in reversed(range(len(intersect))):
# 					i = cube_intersection(intersect[j], off_cube)
# 					if i is not None:
# 						new_r = split_cube(off_cube, i)
# 						del intersect[j]
# 						print()
# 						for v in new_r:
# 							if v not in intersect:
# 								updated = True
# 								print(intersect, v, v not in intersect)
# 								intersect.append(v)
# 		off_cubes += intersect

def intersect_shape_cube(og, off_cubes, cube, e):
	intersect = [cube_intersection(og, cube)]
	# print('INTERSECT')
	# if e:
	# 	print(intersect)
		# exit()
	if intersect[0] is not None:
		updated  = True
		while updated:
			updated = False
			# print('>>>>>>>>>>>>>>')
			for off_cube in off_cubes:
				# print('#')
				for j in reversed(range(len(intersect))):
					i = cube_intersection(intersect[j], off_cube)
					if i is not None:
						new_r = split_cube(intersect[j], i)
						
						# print()
						del intersect[j]
						
						for v in new_r:
							if v not in intersect:
								updated = True
								# print(intersect, v)
								intersect.append(v)
		return intersect

def volume(og, off_cubes):
	v = (og[1][0] - og[0][0]) * (og[1][1] - og[0][1]) * (og[1][2] - og[0][2])
	for off_cube in off_cubes:
		i = cube_intersection(og, off_cube)
		assert i == off_cube
		v -= (off_cube[1][0] - off_cube[0][0]) * (off_cube[1][1] - off_cube[0][1]) * (off_cube[1][2] - off_cube[0][2])
	return v



with open('../../input.txt') as f:

	cuboids = list(map(lambda x: (x.split(' ')[0] == 'on', tuple(map(lambda x: (int(x[2:].split('..')[0]) -1 , int(x[2:].split('..')[1])), x.split()[1].strip().split(',')))), f.readlines()))

	# for x in cuboids:
	# 	print(x)
	bounds = ((-51, -51, -51), (50,50,50))
	cubes = []
	for (i, (is_on, ((x_min, x_max), (y_min, y_max), (z_min, z_max)))) in enumerate(cuboids):
		# print('i', i)
		cube = ((x_min, y_min, z_min), (x_max, y_max, z_max))
		# v = 0
		# for (og, off_cubes) in cubes:
		# 	v += volume(og, off_cubes)
		# print(v)
		# print(cube)
		if is_on:
			intersect = []
			for (og, off_cubes) in cubes:
				x = intersect_shape_cube(og, off_cubes, cube, False)
				if x is not None:
					intersect += x
			cubes.append((cube, intersect))
		else:
			for (og, off_cubes) in cubes:
				x = intersect_shape_cube(og, off_cubes, cube, i == 12)
				if x is not None:
					off_cubes += x
	
	v = 0
	for (og, off_cubes) in cubes:
		i = intersect_shape_cube(og, off_cubes, bounds, False)
		if i is not None:
			for c in i:
				v += volume(c, [])
	print(v)		
						
					