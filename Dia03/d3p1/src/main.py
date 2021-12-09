with open('../../input.txt') as f:
	lines =  f.readlines()
	gamma = '0' * len(lines[0].strip())
	epsilon = '1' * len(gamma)
	count = 0 
	for i in range(len(gamma)):
		for line in lines:
			if line[i] == '1':
				count+=1
				# print(count, len(lines))
				if count>=len(lines)/2:
					gamma = gamma[:i] + '1' + (gamma[i+1:] if i+1 < len(gamma) else '')
					epsilon = epsilon[:i] + '0' + (epsilon[i+1:] if i+1 < len(epsilon) else '')
					break
		# print(gamma)
		count = 0
	print("gamma: " + gamma)
	print("epsilon: " + epsilon)
	print("gamma: " + str(int(gamma,2)))
	print("epsilon: " + str(int(epsilon,2)))
	print('>>> ' + str(int(gamma,2) * int(epsilon,2)))