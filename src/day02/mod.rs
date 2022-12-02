fn get_shape_score(shape: &String) -> u32 {
    match shape.as_str() {
        "ROCK" => 1,
        "PAPER" => 2,
        "SCISSORS" => 3,
        _ => panic!("Invalid shape"),
    }
}

fn get_opponent_shape(opponent_play: &String) -> &str {
    match opponent_play.as_str() {
        "A" => "ROCK",
        "B" => "PAPER",
        "C" => "SCISSORS",
        _ => panic!("Invalid play"),
    }
}

fn get_my_shape(my_play: &String) -> &str {
    match my_play.as_str() {
        "X" => "ROCK",
        "Y" => "PAPER",
        "Z" => "SCISSORS",
        _ => panic!("Invalid play"),
    }
}

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Vec<String>) -> u32 {
    let find_winner = |p1: &String, p2: &String| {
        let p1_val = get_shape_score(p1);
        let p2_val = get_shape_score(p2);

        if ((p1_val % 3) + 1) == p2_val {
            return 1;
        } else if ((p2_val % 3) + 1) == p1_val {
            return -1;
        }

        return 0;
    };

    let mut score = 0;

    for line in input {
        let split: [&str; 2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        match split {
            [opponent_play, my_play] => {
                let op = opponent_play.to_string();
                let mp = my_play.to_string();
                let opponent_shape = get_opponent_shape(&op);
                let my_shape = get_my_shape(&mp);
                let winner = find_winner(&opponent_shape.into(), &my_shape.into());

                let win_score = match winner {
                    0 => 3,
                    1 => 6,
                    _ => 0,
                };

                score += win_score;
                score += get_shape_score(&my_shape.to_string());
            }
        }
    }

    return score;
}
