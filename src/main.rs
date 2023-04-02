mod camera;
mod tokenizer;
mod parser;
mod token;

use std::{collections::HashMap, fmt::format};

use camera::OrbitCamera;
use egui_macroquad::{egui, macroquad::{self, prelude::*}};
use parser::{parse, evaluate};
use token::Token;

/* 
	todo:
	 - play / pause
	 - speed
	 - reset seeds
	 - grid
	 - presets
	 - error handling
 */


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

fn parse_group(e1: &str, e2: &str, e3: &str) -> ([Vec<Token>; 3], HashMap<char, f64>) {
	let (tokens_x, params_x) = parse(e1.to_owned()).unwrap();
	let (tokens_y, params_y) = parse(e2.to_owned()).unwrap();
	let (tokens_z, params_z) = parse(e3.to_owned()).unwrap();

	let mut params = params_x;
	params.extend(params_y);
	params.extend(params_z);

	([tokens_x, tokens_y, tokens_z], params)
}

#[macroquad::main("chaotic attractors")]
async fn main() {
	
	let mut editable_x = "s * (y - x)".to_owned();
	let mut editable_y = "x * (r - z) - y".to_owned();
	let mut editable_z = "x*y - b*z".to_owned();

	let (mut tokens, mut params) = parse_group(&editable_x, &editable_y, &editable_z);

	params.insert('s', 10.0);
	params.insert('r', 28.0);
	params.insert('b', 8.0/3.0);



	// let mut attractor = HashMap::new();

	let mut positions = Vec::new();
	spawn_seeds(&mut positions, 11.0, 1.0, 10.0, 0.1, 13);

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

		for (x, y, z) in &mut positions {
			// let key = (x as i32, y as i32, z as i32);
			// attractor.insert(key, attractor.get(&key).unwrap_or(&0) + 1);
			// lorenz(pos, 10.0, 28.0, 8.0/3.0, 0.01);
			
			params.insert('x', *x);
			params.insert('y', *y);
			params.insert('z', *z);

			let dx = evaluate(&tokens[0], &params).unwrap();
			let dy = evaluate(&tokens[1], &params).unwrap();
			let dz = evaluate(&tokens[2], &params).unwrap();

			*x += dx * 0.01;
			*y += dy * 0.01;
			*z += dz * 0.01;

			draw_cube(vec3(*x as f32, *y as f32, *z as f32), Vec3::splat(0.3), None, ORANGE);
		}

		// for (pos, density) in &attractor {
		// 	draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(1.0), None, Color{r: 0.1, g: 0.2, b: 1.0, a: 1.0});
		// }

		egui_macroquad::ui(|ctx| {
			egui::Window::new("Options").collapsible(false).title_bar(false).fixed_pos((10.0, 10.0)).resizable(false).show(ctx, |ui| {
				ui.heading("Equations");
				let update = 
				ui.horizontal(|ui| {
					ui.label("x' = ");
					ui.text_edit_singleline(&mut editable_x).lost_focus()
				}).inner ||
				ui.horizontal(|ui| {
					ui.label("y' = ");
					ui.text_edit_singleline(&mut editable_y).lost_focus()
				}).inner ||
				ui.horizontal(|ui| {
					ui.label("z' = ");
					ui.text_edit_singleline(&mut editable_z).lost_focus()
				}).inner;

				if update {
					(tokens, params) = parse_group(&editable_x, &editable_y, &editable_z);
				}

				ui.heading("Parameters");
				for (key, value) in params.iter_mut() {
					if *key == 'x' || *key == 'y' || *key == 'z' { continue; }
					ui.horizontal(|ui| {
						ui.label(format!("{key} = "));
						ui.add(egui::DragValue::new(value).speed(0.01)).changed()
					});
				}
			});
		});
		egui_macroquad::draw();
		

		next_frame().await;
	}	
}
