// See https://aka.ms/new-console-template for more information
var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));
var f = 0;
var d = 0;
foreach (string line in lines)
{
	var s = line.Split(' ');
	if(s[0] == "up") {
		d -= Int32.Parse(s[1]);
	}else if(s[0] == "down") {
		d += Int32.Parse(s[1]);
	}else{
		f += Int32.Parse(s[1]);
	}
}
Console.WriteLine(f*d);
