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

fn count_sand(slice: &Vec<Vec<Material>>) -> u32 {
    slice
        .iter()
        .flatten()
        .filter(|it| **it == Material::Sand)
        .count() as u32
}

fn drop_sand(slice: &mut Vec<Vec<Material>>, x_pos: u32, y_pos: u32) -> bool {
    let col = &slice[x_pos as usize];

    for y in (y_pos as usize)..col.len() {
        if y < (col.len() - 1) && col[y + 1] != Material::Air {
            if x_pos > 0 && slice[(x_pos - 1) as usize][y + 1] == Material::Air {
                // Go right
                return drop_sand(slice, x_pos - 1, y as u32);
            } else if x_pos < (slice.len() as u32 - 1)
                && slice[(x_pos + 1) as usize][y + 1] == Material::Air
            {
                // Go left
                return drop_sand(slice, x_pos + 1, y as u32);
            } else {
                slice[x_pos as usize][y] = Material::Sand;
                return true;
            }
        }
    }

    false
}

fn parse_input(input: &[String]) -> Vec<Vec<Material>> {
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
    let mut x_max = u32::MIN;
    let mut y_max = 0;

    for vertices in line_segments.iter() {
        for vertex in vertices {
            x_max = x_max.max(vertex.0);
            y_max = y_max.max(vertex.1);
        }
    }

    let mut slice: Vec<Vec<Material>> =
        vec![vec![Material::Air; (y_max + 1) as usize]; (x_max + 1) as usize];

    for vertices in line_segments {
        for window in vertices.windows(2) {
            let v1 = window[0];
            let v2 = window[1];

            for x in v1.0.min(v2.0)..=v1.0.max(v2.0) {
                for y in v1.1.min(v2.1)..=v1.1.max(v2.1) {
                    slice[x as usize][y as usize] = Material::Rock;
                }
            }
        }
    }

    slice
}

fn print_slice(slice: &Vec<Vec<Material>>) {
    for y in 0..slice[0].len() {
        for x in 0..slice.len() {
            print!(
                "{} ",
                match slice[x][y] {
                    Material::Air => ".",
                    Material::Rock => "#",
                    Material::Sand => "o",
                }
            );
        }

        println!();
    }
}

fn part1(input: &[String]) -> u32 {
    let mut slice = parse_input(input);
    let mut sand_count = 0;

    loop {
        let dropped = drop_sand(&mut slice, 500, 0);

        if !dropped {
            break;
        } else {
            sand_count += 1;
        }
    }

    sand_count
}

fn part2(input: &[String]) -> u32 {
    todo!()
}
