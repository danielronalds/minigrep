use std::env;

fn main() {
    // Collecting the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Assigning arguments to variables
    let query = &args[1]; // We start at index 1 as the 1st index
                          // is taken up by the program name
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

