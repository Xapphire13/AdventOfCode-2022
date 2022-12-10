use core::panic;
use std::collections::VecDeque;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Computer {
    cycle: usize,
    register: i32,
    /** The currently executing instruction */
    instruction: Option<Instruction>,
    program: VecDeque<Instruction>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            cycle: 0,
            register: 1,
            instruction: None,
            program: VecDeque::new(),
        }
    }

    fn tick(&mut self) {
        // Start cycle
        match self.instruction {
            Some(inst) => match inst {
                Instruction::AddX(arg) => {
                    self.register += arg;
                    self.instruction = None;
                }
                _ => panic!("Can't execute instruction"),
            },
            None => {
                let inst = self.program.pop_front();
                self.instruction = inst;
            }
        }

        // End cycle
        self.cycle += 1;

        // Remove Noop at the end of the cycle
        if let Some(inst) = self.instruction {
            if inst == Instruction::Noop {
                self.instruction = None;
            }
        }
    }

    fn load(&mut self, program: &[Instruction]) {
        self.program = VecDeque::from_iter(program.iter().copied())
    }

    fn is_done(&self) -> bool {
        self.instruction.is_none() && self.program.is_empty()
    }

    fn signal(&self) -> i32 {
        (self.cycle as i32) * self.register
    }
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let inst = split.next().unwrap();
            let arg = split.next();

            match inst {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(arg.unwrap().parse().unwrap()),
                _ => panic!("Unsupported instruction"),
            }
        })
        .collect()
}

fn part1(input: &[String]) -> i32 {
    let program = parse_input(input);
    let mut computer = Computer::new();
    computer.load(&program);
    let mut result = 0;

    while !computer.is_done() {
        computer.tick();

        if computer.cycle == 20 || (computer.cycle > 20 && ((computer.cycle - 20) % 40 == 0)) {
            result += computer.signal()
        }
    }

    result
}

fn part2(input: &[String]) -> usize {
    todo!();
}
