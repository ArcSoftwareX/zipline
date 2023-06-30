use std::{env, process};

use zipline::{check_if_gz, compress_file, decompress_file};

fn main() -> Result<(), std::io::Error> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: `operation` `source`");
        process::exit(0)
    }

    if args[0] == "pack" {
        compress_file(&args[1])?;
        println!("done.");
    } else if args[0] == "unpack" {
        if !check_if_gz(&args[1]) {
            eprintln!("File {} is not gzipped", args[1]);
            process::exit(1)
        }
        decompress_file(&args[1])?;
        println!("done.");
    } else {
        eprintln!("Unknown operation: {}", args[0]);
    }

    Ok(())
}
