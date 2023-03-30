use egui_macroquad::egui::epaint::ahash::HashMap;

use crate::tokenizer::{Token, tokenize};

pub fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
	let mut output = Vec::new();
	let mut stack = Vec::new();

	for t in &tokens {
		if t.is_outputable() { // literal, identifier
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

pub fn parse(expression: String) -> Result<(Vec<Token>, HashMap<char, f64>), String> {

	let (infix, parameters) = tokenize(expression)?;
	let postfix = infix_to_postfix(infix)?;

	let parameters = parameters.iter().map(|p| {(*p, 0.0)}).collect::<HashMap<char, f64>>();

	Ok((postfix, parameters))
}