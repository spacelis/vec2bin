extern crate hdf5;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let _args: Vec<String> = env::args().collect();
    let args = parse_args(&_args);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs();
    let (voc, vec, m, n) = parse_vec(&args.src_filename);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs();
    println!("{} seconds, writing to files ...", end - start);
    write_voc(&args.dst_filename, &voc);
    write_vec(&args.dst_filename, &vec, m, n);
    println!("Done!");
}

struct Args {
    src_filename: String,
    dst_filename: String,
}


fn parse_args(args: &[String]) -> Args {
    let src_filename = args[1].clone();
    let dst_filename = args[2].clone();
    Args { src_filename, dst_filename }
}


fn parse_vec(src_filename: &String) -> (Vec<String>, Vec<Vec<f32>>, usize, usize) {
    let mut vec: Vec<Vec<f32>> = Vec::new();
    let mut voc: Vec<String> = Vec::new();
    let file = File::open(src_filename).expect("File not found");
    let mut is_first_line :bool = true;
    let mut m = 0;
    let mut n = 0;
    for (line_no, line) in BufReader::new(file).lines().enumerate() {
        if is_first_line {
            let line = line.unwrap();
            let parts: Vec<usize> = line.split(" ").map(|x| x.parse().unwrap()).collect();
            m = parts[0];
            n = parts[1];
            println!("{} words, {} dimensions", m, n);
            is_first_line = false;
        } else {
            let line = line.unwrap();
            let mut parts: Vec<String> = line.rsplitn(301, " ").map(String::from).collect();
            parts.reverse();
            assert_eq!(parts.len(), 301);
            voc.push(parts[0].clone());
            let vec_part: Vec<f32> = parts[1..301].iter().map(|x| x.parse().unwrap()).collect();
            assert_eq!(vec_part.len(), 300);
            vec.push(vec_part);
        }
        if line_no % 10000 == 0 {
            println!("{} processed", line_no);
        }
    }
    (voc, vec, m, n)
}


fn write_voc(dst_filename: &String, voc: &Vec<String>) {
    let mut file = File::create([dst_filename, ".voc"].join("")).expect("File exists");
    for word in voc.iter() {
        writeln!(file, "{}", word).expect("Write error");
    }
}

fn write_vec(dst_filename: &String, vec: &Vec<Vec<f32>>, m: usize, n: usize) -> () {
    use hdf5::{File, Writer};
    let file = File::new([dst_filename, ".h5"].join("")).unwrap();
    let mut writer = Writer::new(&file, "embedding", &[m, n]);
    let data: Vec<f32> = vec.iter().flat_map(|arr| arr.iter()).cloned().collect();
    println!("{} elements required, {} to write", m*n, data.len());
    writer.write(&data, &[0, 0], &[m, n]).unwrap();
}
