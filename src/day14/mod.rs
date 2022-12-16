use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Eq, PartialEq, Hash)]
struct Position(u32, u32);

struct Slice {
    data: HashMap<Position, Material>,
    height: u32,
    x_min: u32,
    x_max: u32,
    sand_count: u32,
}

impl Slice {
    fn new() -> Slice {
        Slice {
            height: 0,
            x_min: 0,
            x_max: 0,
            sand_count: 0,
            data: HashMap::new(),
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in self.x_min..=self.x_max {
                print!(
                    "{} ",
                    match self.data.get(&Position(x, y)) {
                        Some(Material::Rock) => "#",
                        Some(Material::Sand) => "o",
                        _ => ".",
                    }
                );
            }

            println!();
        }
    }

    fn get(&self, Position(x_pos, y_pos): Position) -> Material {
        *self
            .data
            .get(&Position(x_pos, y_pos))
            .unwrap_or(&Material::Air)
    }

    fn set(&mut self, position: Position, value: Material) {
        self.data.insert(position, value);
    }

    fn drop_sand(&mut self, Position(x_pos, y_pos): Position) -> bool {
        for y in y_pos..self.height {
            if y < (self.height - 1) && self.get(Position(x_pos, y + 1)) != Material::Air {
                if self.get(Position(x_pos - 1, y + 1)) == Material::Air {
                    // Go right
                    return self.drop_sand(Position(x_pos - 1, y));
                } else if self.get(Position(x_pos + 1, y + 1)) == Material::Air {
                    // Go left
                    return self.drop_sand(Position(x_pos + 1, y));
                } else {
                    self.set(Position(x_pos, y), Material::Sand);
                    self.sand_count += 1;
                    return true;
                }
            }
        }

        false
    }
}

fn parse_input(input: &[String]) -> Slice {
    let line_segments: Vec<Vec<(u32, u32)>> = input
        .iter()
        .map(|line_segment| {
            line_segment
                .split(" -> ")
                .map(|vertex| {
                    let mut split = vertex.split(',');

                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    let mut y_max = 0;
    let mut slice = Slice::new();
    let mut x_min = u32::MAX;
    let mut x_max = u32::MIN;

    for vertices in line_segments {
        for window in vertices.windows(2) {
            let v1 = window[0];
            let v2 = window[1];
            y_max = y_max.max(v1.1).max(v2.1);
            x_min = x_min.min(v1.0).min(v2.0);
            x_max = x_max.max(v1.0).max(v2.0);

            for x in v1.0.min(v2.0)..=v1.0.max(v2.0) {
                for y in v1.1.min(v2.1)..=v1.1.max(v2.1) {
                    slice.data.insert(Position(x, y), Material::Rock);
                }
            }
        }
    }

    slice.height = y_max + 1;
    slice.x_min = x_min;
    slice.x_max = x_max;

    slice
}

fn part1(input: &[String]) -> u32 {
    let mut slice = parse_input(input);

    while slice.drop_sand(Position(500, 0)) {}

    slice.print();

    slice.sand_count
}

fn part2(input: &[String]) -> u32 {
    todo!()
}
