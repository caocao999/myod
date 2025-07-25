use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;


fn main() {
    let file_path = get_file_path();

    match run(&file_path) {
        Ok(()) => (),
        Err(err) => {
            eprintln!(" Error : {}",err);
            std::process::exit(1);
        },
    };
}

fn get_file_path()-> String{
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage Error : {}  <file path>",&args[0]);
        std::process::exit(1);
    }
    args[1].to_string()
}

fn run(file_path:&str) -> Result <(), Box<dyn Error>>{
    const   BUFFER_SIZE : usize = 16;
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0u8;BUFFER_SIZE];
    let mut offset = 0;
    
    loop {
        let number_read = file.read(&mut buffer)?;
        if number_read == 0 {
            break;
        }
        print!("{:08x} | ",offset);
        for i in 0..number_read {
            let c = buffer[i];
            print!("{:02x} ",c);
        }
        for _ in number_read..16 {
            print!("   ");
        }
        print!(" | ");
        for i in 0..number_read {
            let c = buffer[i];
            if c.is_ascii_graphic() || c == b' ' {
               print!("{}", c as char); 
            } else {
                print!(".")
            }
        }
        offset += number_read;
        println!();
    }
    Ok(())
}