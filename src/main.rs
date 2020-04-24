use std::io::{BufReader, BufWriter, Read, Write};
use std::fs::File;
use std::env;

fn main() {
    
    // verify correct command-line usage
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() < 1 {
        println!("Usage: ./recover <card file>");
        std::process::exit(1);
    }
    
    // file handler, buffered reader, and buffer
    let f = File::open(&args[0]).expect("Specified input file not found!");
    let mut br = BufReader::new(f);
    let mut buf = [0; 512];

    // identifier counter for all image filenames
    let mut pic_num: u8 = 0;

    // skip leading 0-padding in card data
    while !matches_jpeg_signature(&buf) {
        br.read_exact(&mut buf).expect("Error reading bytes from input file!");
    }

    // read each picture block based on signature
    while matches_jpeg_signature(&buf) {
        pic_num += 1;
        let pic = File::create(format!("{:03}.jpg", pic_num)).expect("Error creating output file!");
        println!("{:03}.jpg created", pic_num);

        let mut bw = BufWriter::new(pic);

        // read card and write to file until we encounter another jpeg or EOF
        loop {
            bw.write_all(&buf).expect("Error writing to file during block parse!");
            match br.read_exact(&mut buf) {
                Ok(_) => if !matches_jpeg_signature(&buf) {} else { break; },
                _ => { break; }
            }
        }
    }
}

fn matches_jpeg_signature(buf: &[u8]) -> bool {
    buf[0..3] == [0xff, 0xd8, 0xff] && (buf[3] & 0b11100000 == 0b11100000)
}