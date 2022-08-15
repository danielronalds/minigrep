use std::{env, fs};

fn main() {
    // Collecting the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Assigning arguments to variables
    let query = &args[1]; // We start at index 1 as the 1st index
                          // is taken up by the program name
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

