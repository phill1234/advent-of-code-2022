namespace AdventOfCode
{
    class Program
    {
        static void Main(string[] args)
        {
            Part1();
            Part2();
        }

        static void Part1()
        {
            char[] letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".ToCharArray();
            int sum = 0;
            // get actual path
            string path = Directory.GetCurrentDirectory();
            // get path to input file
            string inputPath = Path.Combine(path, "input.txt");

            foreach (string line in System.IO.File.ReadLines(inputPath))
            {
                //split line into two parts at half of the length
                string compartments1 = line.Substring(0, line.Length / 2);
                string compartments2 = line.Substring(line.Length / 2);

                // find duplicates in compartments1 and compartments2
                string dublicates = new string(compartments1.Intersect(compartments2).ToArray());

                foreach (char c in dublicates)
                {
                    sum += Array.IndexOf(letters, c) + 1;
                }
            }
            Console.WriteLine($"Part1: {sum}");
        }
        static void Part2()
        {
            int sum = 0;
            // get actual path
            string path = Directory.GetCurrentDirectory();
            // get path to input file
            string inputPath = Path.Combine(path, "input.txt");

            // all lowercase letters as array
            char[] letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".ToCharArray();

            // empty string list
            List<string> entries = new List<string>();


            foreach (string line in System.IO.File.ReadLines(inputPath))
            {
                // add line to entries string list
                entries.Add(line);
                if (entries.Count == 3)
                {
                    char dublicatOfAllThree = FindDuplicate(entries)[0];
                    sum += Array.IndexOf(letters, dublicatOfAllThree) + 1;
                    entries = new List<string>();
                }
            }
            Console.WriteLine($"Part2: {sum}");

        }
        static string FindDuplicate(List<string> entries)
        {
            string[] entriesAsArray = entries.ToArray();
            string matchingDublicate = "";
            // find duplicates in compartments1 and compartments2
            matchingDublicate = new string(entriesAsArray[0].Intersect(entriesAsArray[1]).ToArray());

            if (matchingDublicate == "")
            {
                return "";
            }

            matchingDublicate = new string(matchingDublicate.Intersect(entriesAsArray[2]).ToArray());
            if (matchingDublicate == "")
            {
                return "";
            }
            return matchingDublicate;
        }
    }
}