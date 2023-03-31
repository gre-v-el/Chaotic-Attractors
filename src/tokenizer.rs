use crate::token::{Token, Operator, Function};


pub fn tokenize(mut string: String) -> Result<(Vec<Token>, Vec<char>), String> {
	let mut parameters = vec!['x', 'y', 'z'];
	let mut tokens = Vec::new();
	
	string = string.replace(" ", "");
	string.make_ascii_lowercase();
	
	let mut current = String::new();
	for char in string.chars() {
				
		let mut single_char_token = one_char_token(char);
		if single_char_token == Some(Token::Operator(Operator::Subtract)) {
			if tokens.last() == Some(&Token::Parenthesis(true)) || tokens.last() == Some(&Token::Comma) || tokens.last() == None {
				single_char_token = Some(Token::Operator(Operator::Negate));
			}
		}


		if let Some(token) = single_char_token {
			if !current.is_empty() {
				let previous_token = multichar_token(&current, &mut parameters)?;
				tokens.push(previous_token);
				current.clear();
			}
			tokens.push(token);
		}
		else {
			current.push(char);
		}
	}
	if !current.is_empty() {
		let token = multichar_token(&current, &mut parameters)?;
		tokens.push(token);
	}
	
	
	Ok((tokens, parameters))
}

pub fn one_char_token(string: char) -> Option<Token> {
	match string {
		'+' => Some(Token::Operator(Operator::Add)),
		'-' => Some(Token::Operator(Operator::Subtract)),
		'*' => Some(Token::Operator(Operator::Multiply)),
		'/' => Some(Token::Operator(Operator::Divide)),
		'^' => Some(Token::Operator(Operator::Power)),
		'(' => Some(Token::Parenthesis(true)),
		')' => Some(Token::Parenthesis(false)),
		',' => Some(Token::Comma),
		_  => None
	}
}

pub fn multichar_token(string: &str, parameters: &mut Vec<char>) -> Result<Token, String> {
	let mut alphabetic = 0;
	let mut numeric = 0;
	let mut total = 0;
	let mut example = None;
	for c in string.chars() {
		total += 1;
		if c.is_ascii_alphabetic() { alphabetic += 1 }
		else if c.is_ascii_alphanumeric() || c == '.' { numeric += 1 }
		else { example = Some(c) }
	}
	
	if alphabetic + numeric != total { 
		return Err(format!("Non-ASCII character detected: \"{}\"", if let Some(e) = example {e} else {' '})) 
	}
	if alphabetic == 0 && numeric > 0 {
		let num = if let Ok(n) = string.parse::<f64>() {n} else {return Err(format!("Improper literal: {}", string))};
		return Ok(Token::Literal(num));
	}
	else if alphabetic > 0 && numeric == 0 {
		return Ok(match string {
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
					return Err(format!("Unexpected function: {}", str));
				}
			}
		});
		
	}
	else {
		return Err(format!("Improper literal: {}", string));
	}
}