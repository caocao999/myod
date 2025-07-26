// ============================================================================
// Author: caocao999
// A simple utility similar to UNIX `od` command.
// Displays the contents of a binary file in hexadecimal and ASCII format.
//
// Usage:
//     cargo run -- <file_path>
//
// Example:
//     00000000 | 48 65 6c 6c 6f 20 57 6f 72 6c 64 21             | Hello World!
// ============================================================================

use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;

/// Entry point of the program.
/// Retrieves the file path from arguments and executes the main logic.
fn main() {
    let file_path = get_file_path();

    match run(&file_path) {
        Ok(()) => (),
        Err(err) => {
            eprintln!(" Error : {}", err);
            std::process::exit(1);
        },
    };
}

/// Parses command-line arguments and returns the file path as a string.
/// Exits the program if the argument is missing or incorrect.
fn get_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage Error : {}  <file path>", &args[0]);
        std::process::exit(1);
    }
    args[1].to_string()
}

/// Reads the file contents in 16-byte chunks and prints each chunk
/// in both hexadecimal and ASCII format, emulating a hex dump.
/// Non-printable characters are replaced with '.' in the ASCII output.
fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    const BUFFER_SIZE: usize = 16;
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut offset = 0;

    loop {
        let number_read = file.read(&mut buffer)?;
        if number_read == 0 {
            break;
        }

        // Print offset in hex
        print!("{:08x} | ", offset);

        // Print hexadecimal bytes
        for i in 0..number_read {
            print!("{:02x} ", buffer[i]);
        }

        // Pad with spaces if less than 16 bytes
        for _ in number_read..BUFFER_SIZE {
            print!("   ");
        }

        // Print ASCII characters or '.' for non-printables
        print!(" | ");
        for i in 0..number_read {
            let c = buffer[i];
            if c.is_ascii_graphic() || c == b' ' {
                print!("{}", c as char);
            } else {
                print!(".");
            }
        }

        offset += number_read;
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run(){
        const FILE_PATH:&str = "./src/main.rs";
        let _ = run(FILE_PATH);
    }
}