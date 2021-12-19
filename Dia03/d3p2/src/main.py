with open('../../input.txt') as f:
	lines = list(map(lambda x: x.strip(), f.readlines()))
	gamma = '0' * len(lines[0].strip())
	epsilon = '1' * len(gamma)
	counts = []
	count = 0 
	for i in range(len(gamma)):
		for line in lines:
			
			if line[i] == '1':
				count+=1
				
				# print(count, len(lines))
				if count>=len(lines)/2:
					gamma = gamma[:i] + '1' + (gamma[i+1:] if i+1 < len(gamma) else '')
					epsilon = epsilon[:i] + '0' + (epsilon[i+1:] if i+1 < len(epsilon) else '')
			# print(i, line, count)
		counts.append(count)
		# print(gamma)
		count = 0
	# print(counts)

	co2 = lines.copy()
	co2_counts = counts.copy()
	o2 = lines.copy()
	o2_counts = counts.copy()
	for i in range(len(gamma)+1):
		# print(o2_counts)
		# print(co2_counts)
		# print(i)
		# currcount = o2_counts[i]
		currlen = len(o2) 
		currcounts = o2_counts.copy()
		for j in range(len(o2)-1, -1, -1):
			# print(i,j, o2, o2_counts)
			if len(o2) == 1:
				# print("exiting", o2)
				break
			
			# print(i,o2)
			if currcounts[i] >= currlen/2 and o2[j][i] == '0':
				for x in range(len(o2[j])):
					if o2[j][x] == '1':
						# print("#>",x)
						o2_counts[x] -= 1
				del o2[j]
			elif currcounts[i] < currlen/2 and o2[j][i] == '1':
				for x in range(len(o2[j])):
					if o2[j][x] == '1':
						# print(">>",x)
						o2_counts[x] -= 1
				del o2[j]
		currlen = len(co2)
		currcounts = co2_counts.copy()
		for j in range(len(co2)-1, -1, -1):
			if len(co2) == 1:
				break
			if currcounts[i] >= currlen/2 and co2[j][i] == '1':
				for x in range(len(co2[j])):
					if co2[j][x] == '1':
						co2_counts[x] -= 1
				del co2[j]
			elif currcounts[i] < currlen/2 and co2[j][i] == '0':
				for x in range(len(co2[j])):
					if co2[j][x] == '1':
						co2_counts[x] -= 1
				del co2[j]
			
			# print(o2[j], co2[j])
	print("o2: " + o2[0])
	print("co2: " + co2[0])
	print("o2: " + str(int(o2[0],2)))
	print("co2: " + str(int(co2[0],2)))
	print('>>> ' + str(int(o2[0],2) * int(co2[0],2)))