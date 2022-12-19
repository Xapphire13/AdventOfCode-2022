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

struct Sensor {
    position: Position,
    radius: u32,
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<Position>,
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
            beacons: vec![],
            sensors: vec![],
        };

        for (i, line) in input.iter().enumerate() {
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
        if self
            .beacons
            .iter()
            .any(|beacon_position| *beacon_position == position)
        {
            return MapCell::Beacon;
        }

        for sensor in self.sensors.iter() {
            if sensor.position == position {
                return MapCell::Sensor;
            }

            if sensor.position.abs_distance(position) <= sensor.radius {
                return MapCell::NotBeacon;
            }
        }

        MapCell::Unknown
    }

    fn add_sensor(&mut self, sensor_position: Position, closest_beacon_position: Position) {
        self.sensors.push(Sensor {
            position: sensor_position,
            radius: sensor_position.abs_distance(closest_beacon_position),
        });
        self.beacons.push(closest_beacon_position);
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

        for Position(x, y) in self.beacons.iter() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        for sensor in self.sensors.iter() {
            let sensor_bound_min_x = sensor.position.0 - sensor.radius as i32;
            let sensor_bound_max_x = sensor.position.0 + sensor.radius as i32;
            let sensor_bound_min_y = sensor.position.1 - sensor.radius as i32;
            let sensor_bound_max_y = sensor.position.1 + sensor.radius as i32;

            min_x = min_x.min(sensor_bound_min_x);
            max_x = max_x.max(sensor_bound_max_x);
            min_y = min_y.min(sensor_bound_min_y);
            max_y = max_y.max(sensor_bound_max_y);
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
    let map = Map::parse(input);

    for x in 0u32..=4_000_000u32 {
        for y in 0u32..=4_000_000u32 {
            if map.get(Position(x as i32, y as i32)) == MapCell::Unknown {
                return (x * 4_000_000) + y;
            }
        }
    }

    panic!("Beacon not found!");
}
