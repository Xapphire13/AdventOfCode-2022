use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MapCell {
    Unknown,
    Sensor,
    Beacon,
    NotBeacon,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn abs_distance(&self, other: Position) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

struct Map {
    data: HashMap<Position, MapCell>,
}

impl Map {
    fn parse(input: &[String]) -> Map {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let mut map = Map {
            data: HashMap::new(),
        };

        for line in input {
            let captures = RE.captures(line).unwrap();

            let sensor_x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let sensor_y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

            let sensor = Position(sensor_x, sensor_y);
            let beacon = Position(beacon_x, beacon_y);

            map.add_sensor(sensor, beacon);
        }

        map
    }

    fn get(&self, position: Position) -> MapCell {
        *self.data.get(&position).unwrap_or(&MapCell::Unknown)
    }

    fn set(&mut self, position: Position, value: MapCell) {
        self.data.insert(position, value);
    }

    fn add_sensor(&mut self, sensor_position: Position, closest_beacon_position: Position) {
        self.set(sensor_position, MapCell::Sensor);
        self.set(closest_beacon_position, MapCell::Beacon);
        let distance = sensor_position.abs_distance(closest_beacon_position) as i32;

        for x in (sensor_position.0 - distance)..=(sensor_position.0 + distance) {
            for y in (sensor_position.1 - distance)..=(sensor_position.1 + distance) {
                let position = Position(x, y);

                if self.get(position) == MapCell::Unknown
                    && position.abs_distance(sensor_position) <= (distance as u32)
                {
                    self.set(position, MapCell::NotBeacon);
                }
            }
        }
    }

    fn get_row(&self, y: i32) -> Vec<MapCell> {
        let ((min_x, max_x), _) = self.get_bounds();

        let mut result = vec![];

        for x in min_x..=max_x {
            result.push(self.get(Position(x, y)));
        }

        result
    }

    fn display(&self) {
        let ((min_x, max_x), (min_y, max_y)) = self.get_bounds();

        for y in min_y..=max_y {
            print!("{:>3} ", y);

            for x in min_x..=max_x {
                print!(
                    "{}",
                    match self.get(Position(x, y)) {
                        MapCell::Beacon => 'B',
                        MapCell::Sensor => 'S',
                        MapCell::NotBeacon => '#',
                        MapCell::Unknown => '.',
                    }
                )
            }

            println!();
        }
    }

    fn get_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for Position(x, y) in self.data.keys() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

fn part1(input: &[String]) -> u32 {
    let map = Map::parse(input);

    map.get_row(2_000_000).iter().fold(0, |acc, it| {
        if *it == MapCell::NotBeacon {
            return acc + 1;
        }

        acc
    })
}

fn part2(input: &[String]) -> u32 {
    todo!();
}
