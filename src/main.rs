mod camera;

use std::collections::HashMap;

use camera::OrbitCamera;
use egui_macroquad::{egui::{self, ecolor::rgb_from_hsv}, macroquad::{self, prelude::*}};

fn lorenz(pos: &mut (f64, f64, f64), sigma: f64, rho: f64, beta: f64, dt: f64) {
	let dx = sigma * (pos.1 - pos.0);
	let dy = pos.0 * (rho - pos.2) - pos.1;
	let dz = pos.0 * pos.1 - beta * pos.2;

	pos.0 += dt*dx;
	pos.1 += dt*dy;
	pos.2 += dt*dz;
}

fn spawn_seeds(positions: &mut Vec<(f64, f64, f64)>, cx: f64, cy: f64, cz: f64, radius: f64, amount: usize) {
	for x in 0..amount {
		let x = x as f64 / (amount-1) as f64 * radius*2.0 - radius;

		for y in 0..amount {
			let y = y as f64 / (amount-1) as f64 * radius*2.0 - radius;

			for z in 0..amount {
				let z = z as f64 / (amount-1) as f64 * radius*2.0 - radius;

				positions.push((x + cx, y + cy, z + cz));
			}
		}
	}
}

#[macroquad::main("chaotic attractors")]
async fn main() {
	let mut attractor = HashMap::new();

	let mut positions = Vec::new();
	spawn_seeds(&mut positions, 1.0, 1.0, 10.0, 0.1, 10);

	let mut camera = OrbitCamera {
		center: vec3(1.0, 1.0, 10.0),
		polar: 0.3,
		azimuth: 0.5,
		zoom: -20.0,
		rotate_sinsitivity: 3.0,
		last_mouse: Vec2::from(mouse_position()) / vec2(screen_width(), screen_width()),
	};

	loop {
		clear_background(BLACK);

		camera.update();

		set_camera(&camera.camera());

		for pos in &mut positions {
			let key = (pos.0 as i32, pos.1 as i32, pos.2 as i32);
			attractor.insert(key, attractor.get(&key).unwrap_or(&0) + 1);
			lorenz(pos, 10.0, 28.0, 8.0/3.0, 0.01);
			draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(0.3), None, ORANGE);
		}

		// for (pos, density) in &attractor {
		// 	draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(1.0), None, Color{r: 0.1, g: 0.2, b: 1.0, a: 1.0});
		// }

		next_frame().await;
	}	
}
