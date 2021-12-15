import math

def heuristic(grid, pos):
	return pos[0] + pos[1]

def reconstruct_path(came_from, current):
	x = []
	while current in came_from:
		x.insert(0, current)
		current = came_from[current]
	return x

def path_risk(grid, path):
	return sum(map(lambda x: grid[x[1]][x[0]], path))

def a_star(grid):
	open_set = [(0,0)]
	goal = (len(grid)-1, len(grid[0])-1)
	came_from = {}
	g_score = [[math.inf for _ in range(goal[1]+1)] for _ in range(goal[0]+1)]
	g_score[0][0] = 0

	f_score = [[math.inf for _ in range(goal[1]+1)] for _ in range(goal[0]+1)]
	f_score[0][0] = heuristic(grid, (0,0))

	while len(open_set) > 0:
		current = None
		current_score = math.inf
		for (x, y) in open_set:
			if f_score[y][x] < current_score:
				current = (x, y)
				current_score = f_score[y][x]
		if current == goal:
			return reconstruct_path(came_from, current)
		open_set.remove(current)
		for diff in (-1,1):
			neighbor1 = (current[0] + diff, current[1])
			if neighbor1[0] >= 0 and neighbor1[0] <= goal[0] and neighbor1[1] >= 0 and neighbor1[1] <= goal[1]:
				tentative_g_score = grid[neighbor1[1]][neighbor1[0]] + g_score[current[1]][current[0]]
				if tentative_g_score < g_score[neighbor1[1]][neighbor1[0]]:
					came_from[neighbor1] = current
					g_score[neighbor1[1]][neighbor1[0]] = tentative_g_score
					f_score[neighbor1[1]][neighbor1[0]] = tentative_g_score + heuristic(grid, neighbor1)
					if neighbor1 not in open_set:
						open_set.append(neighbor1)
			neighbor2 = (current[0], current[1] + diff)
			if neighbor2[0] >= 0 and neighbor2[0] <= goal[0] and neighbor2[1] >= 0 and neighbor2[1] <= goal[1]:
				tentative_g_score = grid[neighbor2[1]][neighbor2[0]] + g_score[current[1]][current[0]]
				if tentative_g_score < g_score[neighbor2[1]][neighbor2[0]]:
					came_from[neighbor2] = current
					g_score[neighbor2[1]][neighbor2[0]] = tentative_g_score
					f_score[neighbor2[1]][neighbor2[0]] = tentative_g_score + heuristic(grid, neighbor2)
					if neighbor2 not in open_set:
						open_set.append(neighbor2)
	print("NO PATH FOUND")
	return None



with open('../../input.txt') as f:
	tile = list(map(lambda x: list(map(int, x.strip())), f.readlines()))
	grid = [[None for _ in range(5*len(tile[0]))] for _ in range(5*len(tile))]
	for i in range(5):
		for j in range(5):
			for (y, line) in enumerate(tile):
				for (x, v) in enumerate(line):
					grid[len(tile)*i+y][len(tile[0])*j+x] = (v + i + j)
					if grid[len(tile)*i+y][len(tile[0])*j+x] > 9:
						grid[len(tile)*i+y][len(tile[0])*j+x] -= 9
	# for (y, line) in enumerate(grid):
	# 	for (x, n) in enumerate(line):
	# 		print(str(n), end='')
	# 	print()
	path = a_star(grid)
	# print(path)
	# for (y, line) in enumerate(grid):
	# 	for (x, n) in enumerate(line):
	# 		if (x, y) in path:
	# 			print('#', end='')
	# 		else:
	# 			print('.', end='')
	# 	print()
	
	risk = path_risk(grid, path)
	print(f'>>> {risk}')