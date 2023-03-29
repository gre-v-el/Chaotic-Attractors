use std::{string::ParseError, num::ParseFloatError};

#[derive(Debug)]
pub enum Token {
	Literal(f64),			// 2.0, 3.1415926, 10.0
	Identifier(usize),		// x, y, z, rho, sigma, beta, epsilon (index to a vector)
	Operator(Operator),		// * + - / ^
	Parenthesis(bool),		// (is_opening)
	Function(Function),
	Comma,
}

#[derive(Debug)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Power,
}

#[derive(Debug)]
pub enum Function {
	Sin, Cos, Max, Min, Sign,
}

pub fn tokenize(mut string: String) -> Result<(Vec<Token>, Vec<char>), &'static str> {
	let mut parameters = vec!['x', 'y', 'z'];
	let mut tokens = Vec::new();
	
	string = string.replace(" ", "");
	string.make_ascii_lowercase();
	
	// aggregates characters which cannot be converted to a token by themselves
	let mut current = String::new();
	for char in string.chars() {
		
		let token;
		
		// save literals and function names
		if char.is_ascii_alphanumeric() || char == '.' {
			current.push(char);
			
			token = None;
		}
		
		// operator
		else {
			token = match char {
				'+' => Some(Token::Operator(Operator::Add)),
				'-' => Some(Token::Operator(Operator::Subtract)),
				'*' => Some(Token::Operator(Operator::Multiply)),
				'/' => Some(Token::Operator(Operator::Divide)),
				'^' => Some(Token::Operator(Operator::Power)),
				'(' => Some(Token::Parenthesis(true)),
				')' => Some(Token::Parenthesis(false)),
				',' => Some(Token::Comma),
				_  => None
			};
		}
		
		// if token detected and some literal or function or identifier got aggregated, push it first
		if let Some(token) = token {
			if !current.is_empty() {
				
				let mut alphabetic = 0;
				let mut numeric = 0;
				let mut total = 0;
				let begins_with_letter = current.chars().next().unwrap().is_ascii_alphabetic();
				for c in current.chars() {
					total += 1;
					if c.is_ascii_alphabetic() { alphabetic += 1 }
					else if c.is_ascii_alphanumeric() || c == '.' { numeric += 1 }
				}
				
				if alphabetic + numeric != total { return Err("Non-ASCII characters detected.") }
				if !begins_with_letter && alphabetic > 0 { return Err("Identifier immidiately after literal."); }
				if alphabetic == 0 && numeric > 0 {
					let num = if let Ok(n) = current.parse::<f64>() {n} else {return Err("Improper literal")};
					tokens.push(Token::Literal(num));
				}
				else if alphabetic > 0 && numeric == 0 {
					tokens.push(match current.as_str() {
						"sin"  => Token::Function(Function::Sin),
						"cos"  => Token::Function(Function::Cos),
						"max"  => Token::Function(Function::Max),
						"min"  => Token::Function(Function::Min),
						"sign" => Token::Function(Function::Sign),
						str => {
							if str.len() == 1 {
								let char = str.chars().next().unwrap();
								let mut index = None;
								for (i, p) in parameters.iter().enumerate() {
									if *p == char {
										index = Some(i);
										break;
									}
								}
								let index = if let Some(i) = index { i } else {
									parameters.push(char);
									parameters.len() - 1
								};
								Token::Identifier(index)
							}
							else {
								return Err("Unexpected function");
							}
						}
					});
					
				}
				
				
				current.clear();
			}
			tokens.push(token);
		}
	}
	// if !current.is_empty() {
	// 	println!("{current}");
	// 	let num = if let Ok(n) = current.parse::<f64>() {n} else {return Err("Improper literal")};
	// 	tokens.push(Token::Literal(num));
	// 	current.clear();
	// }
	
	Ok((tokens, parameters))
}