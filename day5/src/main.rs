use std::{io::BufRead, collections::HashMap};

fn main() {
    // read input file line by line
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines();

    // map index to block number e.g 0..3 -> 0, 4..6 -> 1, 7..9 -> 2
    let mut index_to_block = HashMap::new();
    let mut block = 0;
    for i in 0..40 {
        index_to_block.insert(i, block);
        if i % 4 == 0 {
            block += 1;
        }
    }
    
    // create list of 9 lists of chars
    let mut blocks = vec![vec![]; 9];
    for line in lines {
        // if line starts with 1 stop
        let line = line.unwrap();
        if line.starts_with(" 1") {
            break;
        }

        // iterate over chars in line
        let mut index = 0;
        for character in line.chars() {
            if character != ' ' && character != '[' && character != ']' {            
                let index = index_to_block.get(&index).unwrap();
                blocks[*index - 1 as usize].push(character);
            }
            index+=1;
        }

    }

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("move") {
            // get all numbers from line also check if they are two digits
            let mut numbers = vec![];
            for word in line.split_whitespace() {
                if word.parse::<i32>().is_ok() {
                    numbers.push(word.parse::<i32>().unwrap());
                }
            }
            // move amount (numers[0]) of blocks from blocks[numbers[1]] to blocks[numbers[2]]
            for _ in 0..numbers[0] {
                let from = numbers[1] - 1;
                let to = numbers[2] - 1;
                // get and remove first element of block [from]
                let block = blocks[from as usize].remove(0);
                println!("'{}' from {} -> to {}", block, from, to);
                blocks[to as usize].insert(0, block);
                print!("{:?}", blocks);
            }
        }
    }
    // get first entry of every block entry and print it
    for block in blocks {
        println!("{}", block[0]);
    }
}
