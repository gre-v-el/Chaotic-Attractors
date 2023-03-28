use std::collections::HashMap;

use egui_macroquad::{egui::{self, ecolor::rgb_from_hsv}, macroquad::{self, prelude::*}};

fn lorenz(pos: &mut (f64, f64, f64), sigma: f64, rho: f64, beta: f64, dt: f64) {
	let dx = sigma * (pos.1 - pos.0);
	let dy = pos.0 * (rho - pos.2) - pos.1;
	let dz = pos.0 * pos.1 - beta * pos.2;

	pos.0 += dt*dx;
	pos.1 += dt*dy;
	pos.2 += dt*dz;
}

// fn spawn_seeds(position: Vec<(f64, f64, f64)>, x: f64, y: f64, z: f64, )

#[macroquad::main("chaotic attractors")]
async fn main() {
	let mut attractor = HashMap::new();

	let mut positions = Vec::new();

	let mut polar: f32 = 0.5;
	let mut azimuth: f32 = 0.3;
	let mut zoom: f32 = 50.0;

	let sensitivity = 3.0;
	let mut last_mouse = Vec2::from(mouse_position()) / vec2(screen_width(), screen_width());

	let resolution = 20;
	let amplitude = 0.1;
	for x in 0..resolution {
		let x = x as f64 / (resolution-1) as f64 * amplitude*2.0 - amplitude;

		for y in 0..resolution {
			let y = y as f64 / (resolution-1) as f64 * amplitude*2.0 - amplitude;

			for z in 0..resolution {
				let z = z as f64 / (resolution-1) as f64 * amplitude*2.0 - amplitude;

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
		zoom *= 0.999f32.powf(mouse_wheel().1);

		set_camera(&Camera3D {
			position: Vec3 { 
				x: zoom * polar.sin() * azimuth.cos(),
				y: zoom * polar.sin() * azimuth.sin(),
				z: zoom * polar.cos(),
			},
			..Default::default()
		});

		for pos in &mut positions {
			let key = (pos.0 as i32, pos.1 as i32, pos.2 as i32);
			attractor.insert(key, attractor.get(&key).unwrap_or(&0) + 1);
			lorenz(pos, 10.0, 28.0, 8.0/3.0, 0.01);
			draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(0.3), None, ORANGE);
		}

		for (pos, density) in &attractor {
			draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(1.0), None, Color{r: 0.1, g: 0.2, b: 1.0, a: 1.0});
		}

		last_mouse = mouse;
		next_frame().await;
	}	
}
