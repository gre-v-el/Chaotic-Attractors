use std::collections::HashMap;

use egui_macroquad::{egui, macroquad::{self, prelude::*}};

#[macroquad::main("chaotic attractors")]
async fn main() {
	let attractor = HashMap::new();

	let positions = Vec::new();

	let resolution = 4;
	let amplitude = 10.0;
	for x in 0..resolution {
		let x = x as f64 / resolution as f64 * amplitude;

		for y in 0..resolution {
			let y = y as f64 / resolution as f64 * amplitude;

			for z in 0..resolution {
				let z = z as f64 / resolution as f64 * amplitude;

				positions.push((x, y, z));
			}
		}
	}

	loop {
		clear_background(BLACK);

		// set_camera(&Camera3D {

		// });

		next_frame().await;
	}	
}
