// See https://aka.ms/new-console-template for more information
var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));
var f = 0;
var aim = 0;
var d = 0;
foreach (string line in lines)
{
	var s = line.Split(' ');
	if(s[0] == "up") {
		aim -= Int32.Parse(s[1]);
	}else if(s[0] == "down") {
		aim += Int32.Parse(s[1]);
	}else{
		var x = Int32.Parse(s[1]);
		f += x;
		d += aim * x;
	}
}
Console.WriteLine(f*d);
