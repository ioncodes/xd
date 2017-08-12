extern crate flate2;
extern crate clap;

use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let config = config();
    let file_name = config.0;
    let contents = config.1;
    let compress = config.2;
    if compress {
        let mut e = GzEncoder::new(Vec::new(), Compression::Default);
        e.write(&contents[..]).unwrap();
        write_bytes(e.finish().unwrap(), file_name);
    } else {
        let mut d = GzDecoder::new(&contents[..]).unwrap();
        let mut s = Vec::new();
        d.read_to_end(&mut s).unwrap();
        write_bytes(s, file_name);
    }
}

fn write_bytes(data: Vec<u8>, file_name: String) {
    let mut buffer = File::create(file_name).unwrap();
    let _ = buffer.write(&data[..]);
}

fn config() -> (String, Vec<u8>, bool) {
    let matches = App::new("yolo")
        .args(
            &[
                Arg::with_name("compress")
                    .help("compress?")
                    .short("c")
                    .long("compress"),
                Arg::with_name("decompress")
                    .help("decompress?")
                    .short("d")
                    .long("decompress"),
                Arg::with_name("frag")
                    .help("the fragment shader")
                    .index(1)
                    .required(true),
                Arg::with_name("output")
                    .help("name of the output")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .required(true),
            ],
        )
        .get_matches();

    let frag_filename = matches.value_of("frag").unwrap().to_string();
    let output_filename = matches.value_of("output").unwrap().to_string();
    let mut file = File::open(&frag_filename).unwrap();
    let mut contents = Vec::new();
    let _ = file.read_to_end(&mut contents).unwrap();
    let compress = matches.is_present("compress");

    (output_filename, contents, compress)
}
