struct File {
    size: u64,
}

fn main() {
    // read input file line by line
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    // map of file lists in directories
    let mut directories: std::collections::HashMap<String, Vec<File>> =
        std::collections::HashMap::new();
    let mut actual_dir = "/".to_string();

    for line in lines {
        if line.starts_with("$") {
            let command = line.trim_start_matches("$ ");
            run_command(command, &mut actual_dir, &mut directories);
        }
    }
    println!("{}", actual_dir);
}
fn run_command(
    command: &str,
    actual_dir: &mut String,
    directories: &mut std::collections::HashMap<String, Vec<File>>,
) {
    let mut splitted_command_line = command.split_whitespace();
    let command = splitted_command_line.next().unwrap();
    match command {
        "cd" => {
            let dir = splitted_command_line.next().unwrap().to_string();
            *actual_dir = dir;
        }
        "mkdir" => {
            let dir = splitted_command_line.next().unwrap();
            // insert file in in new path
            let file = File { size: 0 };
            let file_list = vec![file];
            directories.insert(dir.to_string(), file_list);
        }
        "touch" => {
            let file = splitted_command_line.next().unwrap();
            let size = splitted_command_line
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let file = File { size };
            let dir = splitted_command_line.next().unwrap();
            directories.get_mut(dir).unwrap().push(file);
        }
        _ => {}
    }
}
