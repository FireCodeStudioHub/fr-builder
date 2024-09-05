use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::env;

// Function to read file and return the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    // Get arguments from command line: input file, output file, and the custom replacement pattern
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input_file> <output_file> <replacement_pattern>", args[0]);
        return Ok(());
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let replacement_pattern = &args[3];

    // Open the output file
    let mut output = File::create(output_file)?;

    // Read the input file line by line
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(mut pattern) = line {
                // Replace "/api/v1/" with the custom pattern
                pattern = pattern.replace("/api/v1/", replacement_pattern);
                
                // Write the modified line to the output file
                writeln!(output, "{}", pattern)?;
            }
        }
    }

    println!("Patterns replaced and saved to {}", output_file);
    Ok(())
}
