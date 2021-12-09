// See https://aka.ms/new-console-template for more information
var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));
// var gamma = (new Array(lines[0].Length)).fill('0');
// var epsilon = (new Array(gamma.Length)).fill('1');
// count = 0;


char[] gamma = new char[lines[0].Count()];
char[] epsilon = new char[gamma.Length];
gamma[0] = '0';
epsilon[0] = '1';
int count = 0;
for (int i = 1; i < gamma.Length; i++)
{
	gamma[i] = '0';
	epsilon[i] = '1';
	foreach (string line in lines)
	{
		// System.out.println(line.trim().charAt(i));
		if (line.Trim()[i] == '1')
		{
			count += 1;
			if (count >= lines.Count() / 2)
			{
				gamma[i] = '1';
				epsilon[i] = '0';
				break;
			}
		}
	}
	count = 0;
}

var g = (Convert.ToInt32(String.Join("", gamma), 2));
var e = (Convert.ToInt32(String.Join("", epsilon), 2));
Console.WriteLine("gamma: " + g.ToString());
Console.WriteLine("epsilon: " + e.ToString());
Console.WriteLine(">>> {0}", e * g);