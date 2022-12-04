use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    const FILENAME: &str = "./input.txt";
    // create empty list
    let mut imp_list: Vec<i32> = Vec::new();
    
    if let Ok(lines) = read_lines(FILENAME) {
        // Consumes the iterator, returns an (Optional) String
        let mut weight_sum_for_one_imp = 0;
        for line in lines {
            if let Ok(line_content) = line {
                if line_content == "" {
                    // empty line, new imp
                    imp_list.push(weight_sum_for_one_imp);
                    weight_sum_for_one_imp = 0;
                } else {
                    // add weight to imp
                    weight_sum_for_one_imp += line_content.parse::<i32>().unwrap();
                }
            }
        }
    }
    // sort list in descending order
    imp_list.sort_by(|a, b| b.cmp(a));
    // get three first entries of list
    let first_three = imp_list.iter().take(3);
    // sum of first three entries
    let sum_of_first_three = first_three.sum::<i32>();

    println!("{}", sum_of_first_three.to_string());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
