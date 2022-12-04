use std::{
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Result, Write},
    path::Path,
};

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut input: String;
    let day: u32;

    loop {
        input = String::new();
        print!("Which day [1-25]: ");
        stdout().flush()?;
        stdin.read_line(&mut input)?;

        match u32::from_str_radix(input.trim(), 10) {
            Ok(value) => {
                day = value;
                break;
            }
            Err(_) => println!("Invalid input: {}", input),
        }
    }

    let path_string = format!("src/day{:0>2}/input.txt", day);
    let path = Path::new(&path_string);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    match day {
        1 => day01::run(lines),
        2 => day02::run(lines),
        3 => day03::run(lines),
        4 => day04::run(lines),
        _ => {}
    }

    Ok(())
}
