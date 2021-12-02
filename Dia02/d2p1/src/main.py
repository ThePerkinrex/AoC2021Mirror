with open('../../input.txt') as f:
	lines =  f.readlines()
	d = 0
	f = 0
	for line in lines:
		[cmd, argstr] = line.split(' ')
		arg = int(argstr)
		if cmd.startswith('up'):
			d -= arg
		elif cmd.startswith('down'):
			d += arg
		else:
			f += arg
	
	print(f*d)