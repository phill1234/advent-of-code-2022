struct File {
    size: u64,
}

fn main() {
    // read input file line by line
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    
    // map of file lists in directories
    let mut directories: std::collections::HashMap<&str, Vec<File>> = std::collections::HashMap::new();
    
    for mut line in lines {
        if line.starts_with("$") {
            line = line.trim_start_matches("$");
            run_command(line);
        }
    }
}

fn run_command(command: &str) {
    println!("Running command: {}", command);
}