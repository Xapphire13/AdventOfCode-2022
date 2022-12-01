use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

mod day01;

fn main() {
    let path = Path::new("src/day01/input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    day01::run(lines);
}
