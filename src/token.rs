#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
	Literal(f64),			// 2.0, 3.1415926, 10.0
	Identifier(usize),		// x, y, z, rho, sigma, beta, epsilon (index to a vector)
	Operator(Operator),		// * + - / ^
	Parenthesis(bool),		// (is_opening)
	Function(Function),
	Comma,
}

impl Token {
	pub fn is_numeric(&self) -> bool {
		match self {
			Self::Literal(_) => true,
			Self::Identifier(_) => true,
			_ => false
		}
	}

	pub fn is_function(&self) -> bool {
		match self {
			Self::Function(_) => true,
			_ => false
		}
	}

	pub fn value(&self, parameters: &Vec<(char, f64)>) -> Result<f64, String> {
		match self {
			Self::Literal(v) => Ok(*v),
			Self::Identifier(c) => if let Some((_, v)) = parameters.get(*c) {Ok(*v)} else {Err("Unknown identifier".into())},
			_ => Err("Unexpected token in evaluation".into())
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
	Add,
	Subtract,
	Negate,
	Multiply,
	Divide,
	Power,
}

impl Operator {
	pub fn left_associative(&self) -> bool {
		match self {
			Self::Power => false,
			_ => true,
		}
	}

	pub fn precedence(&self) -> i8 {
		match self {
			Self::Add => 1,
			Self::Subtract => 1,
			Self::Negate => 1,
			Self::Multiply => 2,
			Self::Divide => 2,
			Self::Power => 3,
		}
	}

	pub fn compare(&self, other: &Self) -> i8 {
		(self.precedence() - other.precedence()).clamp(-1, 1)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
	Sin, Cos, Max, Min, Sign,
}

impl Function {
	pub fn num_arguments(&self) -> usize {
		match self {
			Self::Sin => 1,
			Self::Cos => 1,
			Self::Sign => 1,
			Self::Max => 2,
			Self::Min => 2,
		}
	}
}