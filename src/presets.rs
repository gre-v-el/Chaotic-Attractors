use std::{fs::File, io::{BufReader, BufRead, ErrorKind}};

const PRESETS: &str = ".\\data\\presets.csv";

pub fn read() -> Result<Vec<Preset>, std::io::Error> {

	let mut presets = Vec::new();

	let file = File::open(PRESETS)?;

	for line in BufReader::new(file).lines() {
		let line = line?;
		let mut words = line.split(';');
		let name = if let Some(w) = words.next() { w } else { return Err(ErrorKind::Other.into()) };
		let expr_x = if let Some(w) = words.next() { w } else { return Err(ErrorKind::Other.into()) };
		let expr_y = if let Some(w) = words.next() { w } else { return Err(ErrorKind::Other.into()) };
		let expr_z = if let Some(w) = words.next() { w } else { return Err(ErrorKind::Other.into()) };
		let mut params = Vec::new();
		while let Some(w) = words.next() {
			params.push(if let Ok(n) = w.parse::<f64>() { n } else { return Err(ErrorKind::Other.into()) });
		}

		presets.push(Preset { name: name.into(), expressions: [expr_x.into(), expr_y.into(), expr_z.into()], params });
	}

	Ok(presets)
}

#[derive(Debug)]
pub struct Preset {
	pub name: String, 
	pub expressions: [String; 3],
	pub params: Vec<f64>,
}