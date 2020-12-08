use std::{collections::HashSet, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

struct Program {
    instructions: Vec<Instruction>
}

struct Handheld<'a> {
    accumulator: i32,
    line_number: i32,
    instructions: &'a [Instruction],
}

impl Instruction {
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split_whitespace();
        let operation = Operation::parse(parts.next().ok_or("missing operation")?)?;
        let argument = parts.next().ok_or("missing argument")?.parse::<i32>()?;

        Ok(Self { operation, argument })
    }
}

impl Operation {
    fn parse(input: &str) -> Result<Self> {
        match input {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _ => Err("unknown operation".into()),
        }
    }
}

impl Program {
    fn parse(input: &str) -> Result<Self> {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect::<Result<_>>()?;
        Ok(Self{ instructions })
    }
}

impl<'a> Handheld<'a> {
    fn new(program: &'a Program) -> Self {
        Self{ accumulator: 0, line_number: 0, instructions: &program.instructions }
    }

    fn step(&mut self) -> bool {
        let instruction = self.instructions.get(self.line_number as usize);

        match instruction {
            Some(Instruction{ operation: Operation::Acc, argument }) => {
                self.accumulator += argument;
                self.line_number += 1;
                true
            },
            Some(Instruction{ operation: Operation::Jmp, argument }) => {
                self.line_number += argument;
                true
            },
            Some(Instruction{ operation: Operation::Nop, argument: _ }) => {
                self.line_number += 1;
                true
            },
            None => false,
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let program = Program::parse(&input)?;
    let (accumulator, infinite_looped) = run_program(&program);
    assert!(infinite_looped);
    println!("Part 1: {}", accumulator);

    let mut program = Program::parse(&input)?;

    let result = (0..program.instructions.len()).find_map(|idx| {
        let op = &mut program.instructions[idx].operation;
        let original_op = *op;

        match op {
            Operation::Acc => { return None; },
            Operation::Jmp => { *op = Operation::Nop; }
            Operation::Nop => { *op = Operation::Jmp; }
        }
        
        let (accumulator, infinite_looped) = run_program(&program);

        if infinite_looped {
            program.instructions[idx].operation = original_op;
            return None;
        }

        Some(accumulator)
    });

    println!("Part 2: {}", result.ok_or("no such program exists")?);

    Ok(())
}


fn run_program(program: &Program) -> (i32, bool) {
    let mut handheld = Handheld::new(program);
    let mut lines_visited = HashSet::new();
    let mut infite_looped = false;

    while handheld.step() {
        if !lines_visited.insert(handheld.line_number) {
            infite_looped = true;
            break;
        }
    }

    (handheld.accumulator, infite_looped)
}
