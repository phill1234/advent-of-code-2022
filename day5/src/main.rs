use std::{collections::HashMap, io::BufRead, iter::FromFn, string};

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

        // fill blocks with chars
        let mut index = 0;
        for character in line.chars() {
            if character != ' ' && character != '[' && character != ']' {
                let index = index_to_block.get(&index).unwrap();
                blocks[*index - 1 as usize].push(character);
            }
            index += 1;
        }
    }

    let result_part_1 = move_entries(&mut blocks, "part1");
    print_result(result_part_1);

    let result_part2 = move_entries(&mut blocks, "part2");
    print_result(result_part2);
}

fn move_entries(blocks: &mut Vec<Vec<char>>, mode: &str) -> Vec<Vec<char>> {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines();

    // create copy of blocks
    let mut blocks = blocks.clone();

    println!("{}", mode);
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
            let from = numbers[1] - 1;
            let to = numbers[2] - 1;
            if mode == "part1" {
                for _ in 0..numbers[0] {
                    // get and remove first element of block [from]
                    let block = blocks[from as usize].remove(0);
                    blocks[to as usize].insert(0, block);
                }
            }
            if mode == "part2" {
                let temp_from_blocks = blocks[from as usize].clone();
                for index in (0..numbers[0]).rev() {
                    // get and remove first element of block [from]
                    let block = temp_from_blocks[index as usize];
                    blocks[from as usize].remove(index as usize);
                    blocks[to as usize].insert(0, block);
                }
            }
        }
    }
    return blocks;
}

fn print_result(blocks: Vec<Vec<char>>) {
    // get first entry of every block entry and print it
    for block in blocks {
        if block.len() != 0 {
            println!("{:?}", block[0]);
        }
    }
}
