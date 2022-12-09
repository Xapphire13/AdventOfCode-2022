use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (i32, i32);

struct Rope {
    head: Position,
    tail: Position,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            tail_positions: HashSet::from_iter([(0, 0)]),
        }
    }

    fn move_head(&mut self, direction: Direction, distance: usize) {
        for _ in 0..distance {
            match direction {
                Direction::Up => {
                    self.head.1 += 1;
                }
                Direction::Down => {
                    self.head.1 -= 1;
                }
                Direction::Left => {
                    self.head.0 -= 1;
                }
                Direction::Right => {
                    self.head.0 += 1;
                }
            }

            let dx = self.head.0 - self.tail.0;
            let dy = self.head.1 - self.tail.1;

            // Drifted horizontally
            if dx.abs() == 2 {
                // Diagonal
                if dy.abs() == 1 {
                    self.tail.1 += if dy > 0 { 1 } else { -1 };
                }

                self.tail.0 += if dx > 0 { 1 } else { -1 };
            }
            // Drifted horizontally
            else if dy.abs() == 2 {
                // Diagonal
                if dx.abs() == 1 {
                    self.tail.0 += if dx > 0 { 1 } else { -1 };
                }

                self.tail.1 += if dy > 0 { 1 } else { -1 };
            }

            self.tail_positions.insert(self.tail);
        }
    }
}

fn parse_input(input: &[String]) -> Vec<(Direction, usize)> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let dir = match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = split.next().unwrap().parse::<usize>().unwrap();

            (dir, distance)
        })
        .collect()
}

fn part1(input: &[String]) -> usize {
    let steps = parse_input(input);
    let mut rope = Rope::new();

    for (direction, distance) in steps {
        rope.move_head(direction, distance);
    }

    rope.tail_positions.len()
}

fn part2(input: &[String]) -> usize {
    todo!();
}
