var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));

int count = 0;
for (int i = 0; i < lines.Count(); i++)
{
	string s = lines[i].Trim();
	char[] stack = new char[s.Count()];
	int size = 0;
	for (int j = 0; j < s.Count(); j++)
	{
		char c = s[j];
		if ("([{<".IndexOf(c) != -1)
		{
			stack[size++] = c;
		}
		else
		{
			char x = stack[--size];
			if ((x == '(' && c != ')') || (x != '(' && c - x != 2))
			{
				if (c == ')')
				{
					count += 3;
				}
				else if (c == ']')
				{
					count += 57;
				}
				else if (c == '}')
				{
					count += 1197;
				}
				else if (c == '>')
				{
					count += 25137;
				}
				break;
			}
		}

	}
}
Console.WriteLine(count);
// System.out.println(count);