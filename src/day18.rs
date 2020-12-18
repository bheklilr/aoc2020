use crate::prelude::*;
use meval::tokenizer::{tokenize, Token};

type Int = u128;

pub fn day18() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let hw = parse_part1(&read_file("inputs/day18.txt")?)?;
    answer.part1(eval_and_sum(&hw));
    let hw = parse_part2(&read_file("inputs/day18.txt")?)?;
    answer.part2(eval_and_sum(&hw));
    Ok(answer)
}

fn eval_and_sum(hw: &[Vec<Token>]) -> Int {
    hw.iter().map(|prob| eval(prob)).sum()
}

fn eval(tokens: &[Token]) -> Int {
    use meval::tokenizer::Operation::*;
    use meval::tokenizer::Token::*;

    let mut stack = Vec::new();

    for token in tokens {
        match *token {
            Number(f) => stack.push(f),
            Binary(op) => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                let r = match op {
                    Plus => left + right,
                    Times => left * right,
                    _ => 0.0,
                };
                stack.push(r);
            }
            _ => {}
        }
    }
    stack.pop().expect("Failed to evaluate") as Int
}

fn parse_part1(input: &str) -> R<Vec<Vec<Token>>> {
    input
        .lines()
        .map(|line| tokenize(line).map_err(|e| format!("Invalid expression: {}", e)))
        .collect::<R<Vec<_>>>()?
        .iter()
        .map(|tokens| to_rpn_part1(tokens))
        .collect()
}

fn parse_part2(input: &str) -> R<Vec<Vec<Token>>> {
    input
        .lines()
        .map(|line| tokenize(line).map_err(|e| format!("Invalid expression: {}", e)))
        .collect::<R<Vec<_>>>()?
        .iter()
        .map(|tokens| to_rpn_part2(tokens))
        .collect()
}

fn to_rpn_part1(input: &[Token]) -> R<Vec<Token>> {
    use meval::tokenizer::Token::*;
    use meval::tokenizer::Operation::*;

    let mut output = Vec::with_capacity(input.len());
    let mut stack: Vec<(usize, Token)> = Vec::with_capacity(input.len());

    for (index, token) in input.iter().enumerate() {
        let token = token.clone();
        match token {
            Number(_) => output.push(token),
            Binary(_) => {
                while !stack.is_empty() {
                    match stack.last() {
                        Some((_, Binary(_))) => {
                            output.push(stack.pop().unwrap().1);
                        }
                        _ => break,
                    }
                }
                stack.push((index, token));
            }
            LParen => stack.push((index, token)),
            RParen => {
                let mut found = false;
                while let Some((_, t)) = stack.pop() {
                    match t {
                        LParen => {
                            found = true;
                            break;
                        }
                        _ => output.push(t),
                    }
                }
                if !found {
                    return Err(format!("Mismatched parens at {}", index));
                }
            }
            _ => panic!("Invalid thingy"),
        }
    }
    while let Some((index, token)) = stack.pop() {
        match token {
            Binary(_) => output.push(token),
            _ => panic!(format!(
                "Unexpected token on the stack at position {}",
                index
            )),
        }
    }
    output.shrink_to_fit();
    Ok(output)
}

fn to_rpn_part2(input: &[Token]) -> R<Vec<Token>> {
    use meval::tokenizer::Token::*;
    use meval::tokenizer::Operation::*;

    let mut output = Vec::with_capacity(input.len());
    let mut stack: Vec<(usize, Token)> = Vec::with_capacity(input.len());

    for (index, token) in input.iter().enumerate() {
        let token = token.clone();
        match token {
            Number(_) => output.push(token),
            Binary(_) => {
                while !stack.is_empty() {
                    match stack.last() {
                        Some((_, Binary(Plus))) => {
                            output.push(stack.pop().unwrap().1);
                        }
                        _ => break,
                    }
                }
                stack.push((index, token));
            }
            LParen => stack.push((index, token)),
            RParen => {
                let mut found = false;
                while let Some((_, t)) = stack.pop() {
                    match t {
                        LParen => {
                            found = true;
                            break;
                        }
                        _ => output.push(t),
                    }
                }
                if !found {
                    return Err(format!("Mismatched parens at {}", index));
                }
            }
            _ => panic!("Invalid thingy"),
        }
    }
    while let Some((index, token)) = stack.pop() {
        match token {
            Binary(_) => output.push(token),
            _ => panic!(format!(
                "Unexpected token on the stack at position {}",
                index
            )),
        }
    }
    output.shrink_to_fit();
    Ok(output)
}
