namespace AdventOfCode
{
    class Program
    {
        static void Main(string[] args)
        {
            int sum = 0;
            // get actual path
            string path = Directory.GetCurrentDirectory();
            // get path to input file
            string inputPath = Path.Combine(path, "input.txt");

            // all lowercase letters as array
            char[] letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".ToCharArray();

            foreach (string line in System.IO.File.ReadLines(inputPath))
            {
               //split line into two parts at half of the length
                string compartments1 = line.Substring(0, line.Length / 2);
                string compartments2 = line.Substring(line.Length / 2);

                // get all lower case letters from compartments1
                string lowerCaseCompartments1 = new string(compartments1.Where(c => char.IsLower(c)).ToArray());
                string upperCaseCompartments1 = new string(compartments1.Where(c => char.IsUpper(c)).ToArray());

                // get all lower case letters from compartments2
                string lowerCaseCompartments2 = new string(compartments2.Where(c => char.IsLower(c)).ToArray());
                string upperCaseCompartments2 = new string(compartments2.Where(c => char.IsUpper(c)).ToArray());
                
                // find duplicates in compartments1 and compartments2
                string duplicatesLowerCase = new string(lowerCaseCompartments1.Intersect(lowerCaseCompartments2).ToArray());
                string duplicatesUpperCase = new string(upperCaseCompartments1.Intersect(upperCaseCompartments2).ToArray());

                
                foreach (char c in duplicatesLowerCase)
                {
                    sum += Array.IndexOf(letters, c) + 1;
                }
                foreach (char c in duplicatesUpperCase)
                {
                    sum += Array.IndexOf(letters, c) + 1;
                }                
            }
            Console.WriteLine(sum);
        }
    }
}