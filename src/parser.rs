use std::collections::BTreeMap;

use crate::tokenizer::tokenize;
use crate::token::*;

pub fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
	let mut output = Vec::new();
	let mut stack = Vec::new();

	for t in &tokens {
		if t.is_numeric() {
			output.push(*t);
		}
		else if t.is_function() || *t == Token::Parenthesis(true) {
			stack.push(*t);
		}
		else if let Token::Operator(op1) = t {
			loop {
				if let Some(Token::Operator(op2)) = stack.last() {
					if op2.compare(op1) == 1 || op1.compare(op2) == 0 && op1.left_associative() {
						output.push(stack.pop().unwrap());
					}
					else {
						break;
					}
				}
				else {
					break;
				}
			}
			stack.push(*t);
		}
		else if *t == Token::Parenthesis(false) {
			loop {
				if let Some(Token::Parenthesis(true)) = stack.last() {
					stack.pop();
					if let Some(Token::Function(_)) = stack.last() {
						output.push(stack.pop().unwrap());
					}
					break;
				}
				else {
					if let Some(token) = stack.pop() {
						output.push(token);
					}
					else {
						return Err("Mismatched parentheses".into());
					}
				}
			}
		} 
		else if *t == Token::Comma {
			loop {
				if let Some(Token::Parenthesis(true)) = stack.last() {
					break;
				}
				else {
					if let Some(token) = stack.pop() {
						output.push(token);
					}
					else {
						return Err("Mismatched parentheses".into());
					}
				}
			}
		}
		
	}
	
	while let Some(t) = stack.pop() {
		output.push(t);
	}

	Ok(output)
}

pub fn parse(expression: String) -> Result<(Vec<Token>, BTreeMap<char, f64>), String> {

	let (infix, parameters) = tokenize(expression)?;
	let postfix = infix_to_postfix(infix)?;

	let parameters = parameters.iter().map(|p| {(*p, 0.0)}).collect::<BTreeMap<char, f64>>();

	Ok((postfix, parameters))
}

pub fn evaluate(postfix: &Vec<Token>, parameters: &BTreeMap<char, f64>) -> Result<f64, String> {

	let mut stack = Vec::new();

	for token in postfix {
		if token.is_numeric() {
			stack.push(token.value(&parameters).unwrap());
		}
		else {
			match token {
				Token::Operator(op) => {
					if *op == Operator::Negate {
						let v = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };
						stack.push(-v);
					}
					else {
						let v2 = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };
						let v1 = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };

						let outcome = match op {
							Operator::Add => v1 + v2,
							Operator::Subtract => v1 - v2,
							Operator::Multiply => v1 * v2,
							Operator::Divide => v1 / v2,
							Operator::Power => v1.powf(v2),
							Operator::Negate => 0.0, 		// will not happen
						};

						stack.push(outcome);
					}
				},
				Token::Function(func) => {
					if func.num_arguments() == 1 {
						let v = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };

						let outcome = match func {
							Function::Sin => v.sin(),
							Function::Cos => v.cos(),
							Function::Sign => v.signum(),
							_ => 0.0						// will not happen
						};

						stack.push(outcome);
					}
					else if func.num_arguments() == 2 {
						let v2 = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };
						let v1 = if let Some(v) = stack.pop() { v } else { return Err("Improper expression".into()) };

						let outcome = match func {
							Function::Max => v1.max(v2),
							Function::Min => v1.min(v2),
							_ => 0.0						// will not happen
						};

						stack.push(outcome);
					}
				}
				_ => return Err("Unexpected postfix token".to_owned()),
			}
		}
	}

	if stack.len() == 1{
		Ok(stack.pop().unwrap())
	}
	else {
		Err("Improper expression".into())
	}
}