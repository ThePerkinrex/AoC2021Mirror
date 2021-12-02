import os
import strconv
mut d := 0
mut f := 0
for line in os.read_lines('../../input.txt') or {[]} {
	s := line.split(' ')
	cmd := s[0]
	arg := strconv.atoi(s[1]) or {0}
	if cmd == 'up' {
		d -= arg
	}else if cmd == 'down' {
		d += arg
	}else{
		f += arg
	}
}

println('>>> ${d*f}')