use std::collections::HashMap;

use egui_macroquad::{egui, macroquad::{self, prelude::*}};

#[macroquad::main("chaotic attractors")]
async fn main() {
	// let attractor = HashMap::new();

	let mut positions = Vec::new();

	let mut polar: f32 = 0.5;
	let mut azimuth: f32 = 0.3;
	let mut zoom: f32 = 50.0;

	let sensitivity = 3.0;
	let mut last_mouse = Vec2::from(mouse_position()) / vec2(screen_width(), screen_width());

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
		let mouse = Vec2::from(mouse_position()) / vec2(screen_width(), screen_width());
		clear_background(BLACK);

		if is_mouse_button_down(MouseButton::Left) {
			let delta = mouse - last_mouse;
			azimuth -= delta.x * sensitivity;
			polar -= delta.y * sensitivity;
		}

		set_camera(&Camera3D {
			position: Vec3 { 
				x: zoom * polar.sin() * azimuth.cos(),
				y: zoom * polar.sin() * azimuth.sin(),
				z: zoom * polar.cos(),
			},
			..Default::default()
		});

		for pos in &positions {
			draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(0.3), None, ORANGE);
		}

		last_mouse = Vec2::from(mouse_position()) / vec2(screen_width(), screen_width());
		next_frame().await;
	}	
}
