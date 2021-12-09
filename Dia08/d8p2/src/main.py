with open('../../input.txt') as f:
	sum = 0
	for line in f.readlines():
		digits = list(map(lambda x: list(map(lambda x: ''.join(sorted(x)), x.split())), line.split(' | ')))
		digits[0].sort(key=lambda el: len(el))
		key = ['' for _ in range(10)]
		# print(digits[0])
		key[1] = digits[0][0]
		key[7] = digits[0][1]
		key[4] = digits[0][2]
		key[8] = digits[0][9]
		
		del digits[0][9]
		del digits[0][2]
		del digits[0][1]
		del digits[0][0]
		for i in range(len(digits[0])-1, -1, -1):
			if len(digits[0][i]) == 5:
				if all([char in digits[0][i] for char in key[1]]):
					key[3] = digits[0][i]
					del digits[0][i]
			elif len(digits[0][i]) == 6:
				# print(digits[0][i], "is 9 if", all([char in digits[0][i] for char in key[4]]))
				if all([char in digits[0][i] for char in key[4]]):
					key[9] = digits[0][i]
					del digits[0][i]
				elif all([char in digits[0][i] for char in key[7]]):
					key[0] = digits[0][i]
					del digits[0][i]
				else:
					key[6] = digits[0][i]
					del digits[0][i]
		for i in range(len(digits[0])-1, -1, -1):
			if len(list(filter(lambda x: x not in digits[0][i], key[9]))) == 1:
				key[5] = digits[0][i]
				del digits[0][i]
			else:
				key[2] = digits[0][i]
				del digits[0][i]
		num = int(''.join(map(lambda s: str(key.index(s)), digits[1])))
		# for i in range(len(key)):
		# 	key[i] = sorted(key[i])
		sum += num
		
	print(sum)
