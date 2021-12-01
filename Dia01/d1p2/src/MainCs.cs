using System;
using System.Collections.Generic;

namespace src
{
    class MinCs
    {
        static void Main(string[] args)
        {
            var lines = new List<string>(System.IO.File.ReadLines(@"../../input.txt"));
            var sum = 0;
            for (int i = 3; i < lines.Count; i++)
            {
                if (Int32.Parse(lines[i-3])+Int32.Parse(lines[i-2])+Int32.Parse(lines[i-1]) < Int32.Parse(lines[i-2])+Int32.Parse(lines[i-1])+Int32.Parse(lines[i])) {
                    sum++;
                }
            }
            Console.WriteLine(sum);
        }
    }
}
