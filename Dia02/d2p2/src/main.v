import os
import strconv
mut d := 0
mut f := 0
mut aim := 0
for line in os.read_lines('../../input.txt') or {[]} {
	s := line.split(' ')
	cmd := s[0]
	arg := strconv.atoi(s[1]) or {0}
	if cmd == 'up' {
		aim -= arg
	}else if cmd == 'down' {
		aim += arg
	}else{
		f += arg
		d += aim * arg
	}
}

println('>>> ${d*f}')