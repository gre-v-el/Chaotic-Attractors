mod camera;
mod tokenizer;
mod parser;
mod token;
mod presets;

use std::{collections::BTreeMap, f32::consts::E};

use camera::OrbitCamera;
use egui_macroquad::{egui::{self, RichText, Rgba}, macroquad::{self, prelude::*, rand}};
use parser::{parse, evaluate};
use presets::{read, Preset};
use token::Token;

/* 
	todo:
	 - capture mouse
	 - more presets
 */

fn apply_preset(preset: &Preset) -> Result<([String; 3], [Vec<Token>; 3], BTreeMap<char, f64>), String> {
	let (tokens, mut params) = parse_group(&preset.expressions)?;
	
	let mut index = 0;
	for (c, param) in &mut params {
		if *c == 'x' || *c == 'y' || *c == 'z' { continue; }
		*param = if let Some(v) = preset.params.get(index) {*v} else {return Err("Error while reading a preset".into())};
		index += 1;
	}

	Ok((preset.expressions.clone(), tokens, params))
}

fn parse_group(expression: &[String; 3]) -> Result<([Vec<Token>; 3], BTreeMap<char, f64>), String> {
	let (tokens_x, params_x) = parse(expression[0].clone())?;
	let (tokens_y, params_y) = parse(expression[1].clone())?;
	let (tokens_z, params_z) = parse(expression[2].clone())?;

	let mut params = params_x;
	params.extend(params_y);
	params.extend(params_z);

	Ok(([tokens_x, tokens_y, tokens_z], params))
}



#[macroquad::main("chaotic attractors")]
async fn main() {
	let mut error_msg = "".to_owned();

	let presets = read().unwrap_or_else(|_| {vec![Preset{name: "<There was an error while reading presets>".into(), expressions: ["0".into(), "0".into(), "0".into()], params: vec![]}]});
	let mut selected_preset = 0;
	
	let mut changed = [false; 3];
	let (mut editable, mut tokens, mut params) = apply_preset(&presets[0]).unwrap_or_else(|e| {
		error_msg = e;

		(["0".into(), "0".into(), "0".into()], [vec![Token::Literal(0.0)], vec![Token::Literal(0.0)], vec![Token::Literal(0.0)]], BTreeMap::new())
	});

	let mut playing = true;
	let mut dt = 0.01;

	let mut target_seeds = 1000;
	let mut seed_spawn = (0.0, 0.0, 0.0);
	let mut seed_jitter = 0.1;
	let mut seed_size = 0.3;

	// let mut attractor = HashMap::new();

	let mut seeds = Vec::new();

	let mut camera = OrbitCamera {
		center: vec3(0.0, 0.0, 0.0),
		polar: 2.0,
		azimuth: 1.0,
		zoom: -20.0,
		rotate_sinsitivity: 6.0,
		last_mouse: Vec2::from(mouse_position()) / vec2(screen_width(), screen_width()),
	};

	loop {

		clear_background(BLACK);

		camera.update(true);

		set_camera(&camera.camera());

		if playing {
			while seeds.len() < target_seeds {
				seeds.push((seed_spawn.0 + rand::gen_range(-seed_jitter, seed_jitter), seed_spawn.1 + rand::gen_range(-seed_jitter, seed_jitter), seed_spawn.2 + rand::gen_range(-seed_jitter, seed_jitter)));
			}
			if seeds.len() > target_seeds {
				seeds.drain(target_seeds..);
			}

			for (x, y, z) in &mut seeds {
				// let key = (x as i32, y as i32, z as i32);
				// attractor.insert(key, attractor.get(&key).unwrap_or(&0) + 1);
				// lorenz(pos, 10.0, 28.0, 8.0/3.0, 0.01);
				
				params.insert('x', *x);
				params.insert('y', *y);
				params.insert('z', *z);
	
				
				let dx = evaluate(&tokens[0], &params).unwrap_or_else(|e| {error_msg=e; 0.0});
				let dy = evaluate(&tokens[1], &params).unwrap_or_else(|e| {error_msg=e; 0.0});
				let dz = evaluate(&tokens[2], &params).unwrap_or_else(|e| {error_msg=e; 0.0});
	
				*x += dx * dt;
				*y += dy * dt;
				*z += dz * dt;
	
			}
		}

		draw_line_3d((-1000.0, 0.0, 0.0).into(), (1000.0, 0.0, 0.0).into(), GRAY);
		draw_line_3d((0.0, -1000.0, 0.0).into(), (0.0, 1000.0, 0.0).into(), GRAY);
		draw_line_3d((0.0, 0.0, -1000.0).into(), (0.0, 0.0, 1000.0).into(), GRAY);
		for (x, y, z) in &mut seeds {
			draw_cube(vec3(*x as f32, *y as f32, *z as f32), Vec3::splat(seed_size), None, ORANGE);
		}

		// for (pos, density) in &attractor {
		// 	draw_cube(vec3(pos.0 as f32, pos.1 as f32, pos.2 as f32), Vec3::splat(1.0), None, Color{r: 0.1, g: 0.2, b: 1.0, a: 1.0});
		// }

		let mut apply = false;
		egui_macroquad::ui(|ctx| {
			egui::Window::new("Options").collapsible(false).title_bar(false).fixed_pos((10.0, 10.0)).resizable(false).show(ctx, |ui| {
				ui.label(RichText::new(&error_msg).color(Rgba::from_rgb(1.0, 0.0, 0.0)));

				ui.heading("Controls");
				playing = playing ^ ui.button(if playing { "pause" } else { "play" }).clicked();
				ui.horizontal(|ui| {
					ui.add(egui::DragValue::new(&mut dt).speed(0.001));
					ui.label("  dt");
				});
				ui.horizontal(|ui| {
					ui.add(egui::DragValue::new(&mut target_seeds).speed(10));
					ui.label("  number of seeds");
				});
				ui.horizontal(|ui| {
					ui.add(egui::DragValue::new(&mut seed_size).speed(0.001));
					ui.label("  seed size");
				});
				ui.horizontal(|ui| {
					ui.add(egui::DragValue::new(&mut seed_jitter).speed(0.001));
					ui.label("  seed placement jitter");
				});
				
				ui.horizontal(|ui| {
					ui.add(egui::DragValue::new(&mut seed_spawn.0).speed(0.1));
					ui.add(egui::DragValue::new(&mut seed_spawn.1).speed(0.1));
					ui.add(egui::DragValue::new(&mut seed_spawn.2).speed(0.1));
					ui.label("  seed spawn position");
				});


				ui.add_space(20.0);
				ui.heading("Equations");
				if egui::ComboBox::from_label("Presets").show_index(ui, &mut selected_preset, presets.len(), |i| {presets[i].name.clone()}).changed() {
					(editable, tokens, params) = apply_preset(&presets[selected_preset]).unwrap_or_else(|e| {
						error_msg = e;

						(["0".into(), "0".into(), "0".into()], [vec![Token::Literal(0.0)], vec![Token::Literal(0.0)], vec![Token::Literal(0.0)]], BTreeMap::new())
					}); 
				}

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
			(tokens, params) = parse_group(&editable).unwrap_or_else(|e| {
				error_msg = e;
				([vec![Token::Literal(0.0)], vec![Token::Literal(0.0)], vec![Token::Literal(0.0)]], BTreeMap::new())
			});
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
