use std::io::{self, Read};
use filter_lib::exclude_lines;

fn main() {
    // Read from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    // Get command line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // Apply exclude
    let result = exclude_lines(input, &arg_refs);

    // Print result
    print!("{}", result);
}
