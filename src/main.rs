mod camera;
mod tokenizer;
mod parser;
mod token;

use std::collections::BTreeMap;

use camera::OrbitCamera;
use egui_macroquad::{egui, macroquad::{self, prelude::*}};
use parser::{parse, evaluate};
use token::Token;

/* 
	todo:
	 - reset seeds
	 - escape radius
	 - capture mouse
	 - presets
	 - error handling
 */

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

fn parse_group(expression: &[String; 3]) -> ([Vec<Token>; 3], BTreeMap<char, f64>) {
	let (tokens_x, params_x) = parse(expression[0].clone()).unwrap();
	let (tokens_y, params_y) = parse(expression[1].clone()).unwrap();
	let (tokens_z, params_z) = parse(expression[2].clone()).unwrap();

	let mut params = params_x;
	params.extend(params_y);
	params.extend(params_z);

	([tokens_x, tokens_y, tokens_z], params)
}

#[macroquad::main("chaotic attractors")]
async fn main() {
	
	let mut changed = [false; 3];
	let mut editable = ["s * (y - x)".into(), "x * (r - z) - y".into(), "x*y - b*z".into()];

	let (mut tokens, mut params) = parse_group(&editable);

	params.insert('s', 10.0);
	params.insert('r', 28.0);
	params.insert('b', 8.0/3.0);

	let mut playing = true;
	let mut dt = 0.01;


	// let mut attractor = HashMap::new();

	let mut positions = Vec::new();
	spawn_seeds(&mut positions, 11.0, 1.0, 10.0, 0.1, 13);

	let mut camera = OrbitCamera {
		center: vec3(1.0, 1.0, 10.0),
		polar: 0.3,
		azimuth: 0.5,
		zoom: -20.0,
		rotate_sinsitivity: 6.0,
		last_mouse: Vec2::from(mouse_position()) / vec2(screen_width(), screen_width()),
	};

	loop {

		clear_background(BLACK);

		camera.update();

		set_camera(&camera.camera());

		if playing {
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
	
				*x += dx * dt;
				*y += dy * dt;
				*z += dz * dt;
	
			}
		}

		draw_line_3d((-1000.0, 0.0, 0.0).into(), (1000.0, 0.0, 0.0).into(), GRAY);
		draw_line_3d((0.0, -1000.0, 0.0).into(), (0.0, 1000.0, 0.0).into(), GRAY);
		draw_line_3d((0.0, 0.0, -1000.0).into(), (0.0, 0.0, 1000.0).into(), GRAY);
		for (x, y, z) in &mut positions {
			draw_cube(vec3(*x as f32, *y as f32, *z as f32), Vec3::splat(0.3), None, ORANGE);
		}

		// for (pos, density) in &attractor {
		// 	draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(1.0), None, Color{r: 0.1, g: 0.2, b: 1.0, a: 1.0});
		// }

		let mut apply = false;
		egui_macroquad::ui(|ctx| {
			egui::Window::new("Options").collapsible(false).title_bar(false).fixed_pos((10.0, 10.0)).resizable(false).show(ctx, |ui| {
				ui.heading("Controls");
				playing = playing ^ ui.button(if playing { "pause" } else { "play" }).clicked();
				ui.add(egui::DragValue::new(&mut dt).speed(0.0001));

				ui.add_space(20.0);
				ui.heading("Equations");
				let chars = ['x', 'y', 'z'];
				for i in 0..3 {
					changed[i] |= ui.horizontal(|ui| {
						ui.label(format!("{}' = ", chars[i]));
						let resp = ui.text_edit_singleline(&mut editable[i]).changed();
						if changed[i] {
							ui.label("[ ! ]").on_hover_text("Edited - to apply click \"apply\"");
						}
						resp
					}).inner;
				}

				apply = ui.button("apply").clicked();

				ui.add_space(20.0);
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
		if apply {
			let old_params = params;
			(tokens, params) = parse_group(&editable);
			for (k, v) in &mut params {
				if let Some(old_v) = old_params.get(&k) {
					*v = *old_v;
				}
			}
			changed = [false; 3];
		}
		

		next_frame().await;
	}	
}
