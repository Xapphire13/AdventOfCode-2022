use std::{collections::VecDeque, vec};

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Copy, Clone)]
struct Item(u32);

struct Test {
    divisor: u32,
    on_pass: u32,
    on_fail: u32,
}

impl Test {
    fn execute(&self, worry_score: u32) -> u32 {
        if worry_score % self.divisor == 0 {
            self.on_pass
        } else {
            self.on_fail
        }
    }
}

enum Operand {
    UseSelf,
    Value(u32),
}

enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl Operation {
    fn apply(&self, other: u32) -> u32 {
        let get_value = |operand: &Operand| match operand {
            Operand::Value(val) => *val,
            Operand::UseSelf => other,
        };

        match self {
            Operation::Add(operand) => get_value(operand) + other,
            Operation::Multiply(operand) => get_value(operand) * other,
        }
    }
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspections_made: u32,
}

impl Monkey {
    fn take_turn(&mut self, divisor: u32, mod_val: u32) -> Vec<(Item, usize)> {
        let mut result = vec![];

        while let Some(Item(worry_score)) = self.items.pop_front() {
            let new_worry_score = (self.operation.apply(worry_score) % mod_val) / divisor;
            let next_monkey = self.test.execute(new_worry_score);

            result.push((Item(new_worry_score), next_monkey as usize));
            self.inspections_made += 1;
        }

        result
    }
}

fn parse_input(input: &[String]) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut iter = input.iter();

    while let Some(_) = iter.next() {
        let items = iter.next().unwrap().split(": ").nth(1).unwrap().split(", ");
        let items: VecDeque<Item> = items
            .map(|item| Item(item.parse::<u32>().unwrap()))
            .collect();
        let operation = iter.next().unwrap().split("= old ").nth(1).unwrap();
        let operation = match operation.chars().next() {
            Some('*') => Operation::Multiply(match operation.split("* ").nth(1).unwrap() {
                "old" => Operand::UseSelf,
                other => Operand::Value(other.parse().unwrap()),
            }),
            Some('+') => Operation::Add(match operation.split("+ ").nth(1).unwrap() {
                "old" => Operand::UseSelf,
                other => Operand::Value(other.parse().unwrap()),
            }),
            Some(_) => panic!("Unexpected operation"),
            None => panic!(),
        };
        let divisor = iter
            .next()
            .unwrap()
            .split("divisible by ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let true_monkey = iter
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let false_monkey = iter
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        monkeys.push(Monkey {
            inspections_made: 0,
            items,
            operation,
            test: Test {
                divisor,
                on_pass: true_monkey,
                on_fail: false_monkey,
            },
        });

        // Consume empty line
        iter.next();
    }

    monkeys
}

fn part1(input: &[String]) -> u32 {
    let mut monkeys = parse_input(input);

    // 20 rounds
    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let result = monkeys[i].take_turn(3, u32::MAX);

            for (item, next_monkey) in result.iter() {
                monkeys[*next_monkey].items.push_back(*item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections_made.cmp(&a.inspections_made));

    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspections_made)
}

fn part2(input: &[String]) -> u32 {
    let mut monkeys = parse_input(input);
    let mod_val = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test.divisor);

    // 10,000 rounds
    for _ in 1..=10_000 {
        for i in 0..monkeys.len() {
            let result = monkeys[i].take_turn(1, mod_val);

            for (item, next_monkey) in result.iter() {
                monkeys[*next_monkey].items.push_back(*item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections_made.cmp(&a.inspections_made));

    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, monkey| (acc * monkey.inspections_made) % mod_val)
}
