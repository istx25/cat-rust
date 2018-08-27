use std::env;
use std::io::{self, Read, Write};
use std::iter;
use std::fs::File;

const SMALL_BUFFER_SIZE: usize = 256;
const LARGE_BUFFER_SIZE: usize = 64 * 1024;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        args.push("-".into());
    }

    let stdout = &mut io::stdout();
    cat(args, stdout);
}

fn cat (args: Vec<String>, stdout: &mut io::Stdout) {
    let buffer = &mut vec![0; SMALL_BUFFER_SIZE];

    for arg in args {
        if "-" == arg {
            if let Err(error) = redirect_stream(&mut io::stdin(), stdout, buffer) {
                println!("{}", error.to_string());
            }

            continue;
        }

        match File::open(arg) {
            Ok(ref mut file) => {
                if let Err(error) = redirect_stream(file, stdout, buffer) {
                    println!("{}", error.to_string());
                }
            },
            Err(error) => {
                println!("{}", error);

                continue;
            }
        }
    }
}

fn redirect_stream<R, W>(reader: &mut R, writer: &mut W, buffer: &mut Vec<u8>) -> io::Result<()> where R: Read, W: Write {
    loop {
        let len_read = try!(reader.read(buffer));

        if 0 == len_read {
            return Ok(());
        }

        try!(writer.write_all(&buffer[..len_read]));

        if len_read == buffer.len() && len_read < LARGE_BUFFER_SIZE {
            buffer.extend(iter::repeat(0).take(len_read));
        }
    }
}
