// fview program modified to print also the last line which may be < BYTES_PER_LINE.

use std::env;
use std::fs::File;
use std::io::Read;

const BYTES_PER_LINE: usize = 16; // <1>

fn main() {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    // possible after 1.65 / #![feature(let_chains)]?
    // while let Ok(n) = f.read(&mut buffer) && n > 0 {

    while let Ok(n) = f.read(&mut buffer) {
        if n == 0 {
            break;
        }
        print!("[0x{:08x}] ", pos);
        for b in buffer.iter().take(n) {
            match b {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", b),
            }
        }

        println!();
        pos += n;
    }
}

//// original code
//   while f.read_exact(&mut buffer).is_ok() {
//     print!("[0x{:08x}] ", pos);
//     for byte in &buffer {
//       match *byte {
//         0x00 => print!(".  "),
//         0xff => print!("## "),
//         _ => print!("{:02x} ", byte),
//       }
//     }

//     println!();
//     pos += BYTES_PER_LINE;
//   }
