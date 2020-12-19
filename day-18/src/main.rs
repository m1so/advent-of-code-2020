#![feature(or_patterns)]

use std::{collections::{VecDeque, vec_deque::Iter}, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Value(u64),
    Mult,
    Add,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Eq)]
enum Ast {
    Value(u64),
    Op(Operation),
    Group(VecDeque<Ast>)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Mult,
    Add,
}

impl Ast {
    fn from_tokens(tokens: &mut VecDeque<Token>) -> Ast {
        let mut ast: VecDeque<Ast> = VecDeque::new();

        while !tokens.is_empty() {
            match tokens.pop_front() {
                // handle scalars and operators
                Some(Token::Value(val)) => ast.push_back(Ast::Value(val)),
                Some(Token::Mult) => ast.push_back(Ast::Op(Operation::Mult)),
                Some(Token::Add) => ast.push_back(Ast::Op(Operation::Add)),
                // recurse into content of the parens
                Some(Token::LParen) => ast.push_back(Ast::from_tokens(tokens)),
                // stop recursing
                Some(Token::RParen) => return Ast::Group(ast),
                None => {}, // tokens fully consumed = nop
            }
        }

        Ast::Group(ast) // root the expression
    }
}

fn tokenize(line: &str) -> VecDeque<Token> {
    line.chars().filter_map(|c| {
        match c {
            '*' => Some(Token::Mult),
            '+' => Some(Token::Add),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            c if c.is_digit(10) => Some(Token::Value(c.to_digit(10).unwrap().into())),
            _ => None,
        }
    }).collect::<VecDeque<Token>>()
}

fn evaluate<F>(ast: &Ast, group_fn: &mut F) -> u64 
where F: FnMut(&mut Iter<Ast>, &mut u64) -> u64 {
    match ast {
        Ast::Value(val) => *val,
        Ast::Op(_) => unreachable!(),
        Ast::Group(elements) if elements.len() >= 1 => {
            let mut elem_iter = elements.iter();
            let mut current = evaluate(elem_iter.next().unwrap(), group_fn);

            group_fn(&mut elem_iter, &mut current)
        },
        _ => unreachable!(),
    }
}

fn evaluate_op(op: &Operation, left: u64, right: u64) -> u64 {
    match op {
        Operation::Mult => left * right,
        Operation::Add => left + right,
    }
}

fn evaluate_group(elem_iter: &mut Iter<Ast>, current: &mut u64) -> u64 {
    loop {
        match (elem_iter.next(), elem_iter.next()) {
            (
                Some(Ast::Op(op)), 
                Some(next @ (Ast::Group(_) | Ast::Value(_)))
            ) => {
                *current = evaluate_op(op, *current, evaluate(next, &mut evaluate_group));
            },
            _ => break *current,
        }
    }
}

fn evaluate_group_addition_first(elem_iter: &mut Iter<Ast>, current: &mut u64) -> u64 {
    let mut multiplication_ast: Vec<Ast> = Vec::new();
    
    loop {
        match (elem_iter.next(), elem_iter.next()) {
            (
                Some(Ast::Op(op)), 
                Some(next @ (Ast::Group(_) | Ast::Value(_)))
            ) => {
                let next_val = evaluate(next, &mut evaluate_group_addition_first);
                
                *current = match op {
                    Operation::Add => *current + next_val,
                    Operation::Mult => {
                        multiplication_ast.push(Ast::Value(*current));
                        multiplication_ast.push(Ast::Op(Operation::Mult));
                        
                        next_val
                    },
                };
            },
            _ => { multiplication_ast.push(Ast::Value(*current)); break; },
        }
    }

    if multiplication_ast.len() == 0 {
        return *current;
    }

    let mut iter = multiplication_ast.iter();
    let mut intermediate_multiple = evaluate(iter.next().unwrap(), &mut evaluate_group_addition_first);
    
    loop {
        match (iter.next(), iter.next()) {
            (
                Some(Ast::Op(op)), 
                Some(next @ (Ast::Group(_) | Ast::Value(_)))
            ) => {
                intermediate_multiple = match op {
                    Operation::Add => unreachable!(),
                    Operation::Mult => intermediate_multiple * evaluate(next, &mut evaluate_group_addition_first),
                };
            },
            _ => { *current = intermediate_multiple; break *current; },
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let homework_result: u64 = input
        .lines()
        .map(|line| evaluate(&Ast::from_tokens(&mut tokenize(line)), &mut evaluate_group))
        .sum();

    println!("Part 1: {}", homework_result);

    let homework_result: u64 = input
        .lines()
        .map(|line| evaluate(&Ast::from_tokens(&mut tokenize(line)), &mut evaluate_group_addition_first))
        .sum();

    println!("Part 2: {}", homework_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_samples() {
        assert_eq!(evaluate(&Ast::from_tokens(&mut tokenize("1 + 2 * 3 + 4 * 5 + 6")), &mut evaluate_group), 71);
        assert_eq!(evaluate(&Ast::from_tokens(&mut tokenize("1 + (2 * 3) + (4 * (5 + 6))")), &mut evaluate_group), 51);
    }

    #[test]
    fn test_part_2_samples() {
        assert_eq!(evaluate(&Ast::from_tokens(&mut tokenize("1 + 2 * 3 + 4 * 5 + 6")), &mut evaluate_group_addition_first), 231);
        assert_eq!(evaluate(&Ast::from_tokens(&mut tokenize("1 + (2 * 3) + (4 * (5 + 6))")), &mut evaluate_group_addition_first), 51);
    }
}
