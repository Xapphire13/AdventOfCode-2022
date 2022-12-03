pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn get_shape_score(shape: Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_opponent_shape(opponent_play: &str) -> Shape {
    match opponent_play {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid play!"),
    }
}

fn get_my_shape(my_play: &str) -> Shape {
    match my_play {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Invalid play!"),
    }
}

fn play(opponent_shape: Shape, my_shape: Shape) -> u32 {
    let opponent_shape_val = get_shape_score(opponent_shape);
    let my_shape_val = get_shape_score(my_shape);
    let result = if ((opponent_shape_val % 3) + 1) == my_shape_val {
        6 // We win
    } else if ((my_shape_val % 3) + 1) == opponent_shape_val {
        0 // We lose
    } else {
        3 // Tie
    };

    return result + my_shape_val;
}

fn part1(input: &Vec<String>) -> u32 {
    let mut score = 0;

    for line in input {
        let split: [&str; 2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        match split {
            [opponent_play, my_play] => {
                score += play(get_opponent_shape(opponent_play), get_my_shape(my_play))
            }
        }
    }

    return score;
}
