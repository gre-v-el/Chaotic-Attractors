use std::{string::ParseError, num::ParseFloatError};

// todo: validate(), sin/max/min/sign, ','
#[derive(Debug)]
pub enum Token {
	Literal(f64),			// 2.0, 3.1415926, 10.0
	Parameter(usize),		// x, y, z, rho, sigma, beta, epsilon (index to a vector)
	Operator(Operator),		// * + - / ^
	Parenthesis(bool),		// (is_opening)
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
	Sin, Cos, Max, Min,
}

pub fn tokenize(mut string: String) -> Result<(Vec<Token>, Vec<char>), ParseFloatError> {
	let mut parameters = vec!['x', 'y', 'z'];
	let mut tokens = Vec::new();

	string = string.replace(" ", "");
	string.make_ascii_lowercase();

	// aggregates characters which cannot be converted to a token by themselves
	let mut current = String::new();
	for char in string.chars() {

		let token;
		// parameter
		if char.is_ascii_alphabetic() {
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
			token = Some(Token::Parameter(index))
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

		// if token detected and some literal got aggregated, push it first
		if let Some(token) = token {
			if !current.is_empty() {
				let num = current.parse::<f64>()?;
				tokens.push(Token::Literal(num));
				current.clear();
			}
			tokens.push(token);
		}
		else {
			current.push(char);
		}
	}
	if !current.is_empty() {
		let num = current.parse::<f64>()?;
		tokens.push(Token::Literal(num));
		current.clear();
	}

	Ok((tokens, parameters))
}