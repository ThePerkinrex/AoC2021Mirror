with open('../../input.txt') as f:
	lines =  f.readlines()
	d = 0
	f = 0
	aim = 0
	for line in lines:
		[cmd, argstr] = line.split(' ')
		arg = int(argstr)
		if cmd.startswith('up'):
			aim -= arg
		elif cmd.startswith('down'):
			aim += arg
		else:
			f += arg
			d += arg * aim
	
	print(f*d)