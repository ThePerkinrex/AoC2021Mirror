// See https://aka.ms/new-console-template for more information
var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));
// var gamma = (new Array(lines[0].Length)).fill('0');
// var epsilon = (new Array(gamma.Length)).fill('1');
// count = 0;


char[] gamma = new char[lines[0].Count()];
char[] epsilon = new char[gamma.Length];
// gamma[0] = '0';
// epsilon[0] = '1';
int count = 0;
int[] counts = new int[gamma.Length];
for (int i = 0; i < gamma.Length; i++)
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
			}
		}
	}
	counts[i] = count;
	count = 0;
}

List<string> co2 = new List<string>(lines);
int[] co2_counts = new int[counts.Length];
Array.Copy(counts, co2_counts, counts.Length);
List<string> o2 = new List<string>(lines);
int[] o2_counts = new int[counts.Length];
Array.Copy(counts, o2_counts, counts.Length);
for (int i = 0; i <= gamma.Length; i++)
{
	int currlen = o2.Count();
	int[] currcounts = new int[counts.Length];
	Array.Copy(o2_counts, currcounts, counts.Length);
	for (int j = o2.Count() -1; j >= 0 && o2.Count() > 1; j--)
	{
		if ((((float) currcounts[i]) >= ((float) currlen) / 2f && o2[j][i] == '0') || (((float) currcounts[i]) < ((float) currlen) / 2f && o2[j][i] == '1')) {
			for (int x = 0; x < o2[j].Length; x++)
			{
				if (o2[j][x] == '1') {
					o2_counts[x] -= 1;
				}
			}
			o2.RemoveAt(j);
		}
	}
	currlen = co2.Count();
	Array.Copy(co2_counts, currcounts, counts.Length);
	for (int j = co2.Count() -1; j >= 0 && co2.Count() > 1; j--)
	{
		if ((((float) currcounts[i]) >= ((float) currlen) / 2f && co2[j][i] == '1') || (((float) currcounts[i]) < ((float) currlen) / 2f && co2[j][i] == '0')) {
			for (int x = 0; x < co2[j].Length; x++)
			{
				if (co2[j][x] == '1') {
					co2_counts[x] -= 1;
				}
			}
			co2.RemoveAt(j);
		}
	}
}

var g = (Convert.ToInt32(o2[0], 2));
var e = (Convert.ToInt32(co2[0], 2));
Console.WriteLine(" o2: " + g.ToString());
Console.WriteLine("co2: " + e.ToString());
Console.WriteLine(">>> {0}", e * g);