extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate noise;

use std::f64::consts::PI;

use piston::window::WindowSettings;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::*;
use graphics::*;
use graphics::types::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use noise::{Perlin, RotatePoint, NoiseFn};

type Triangle = [[f64; 2]; 3];

/*
1: Spiral
2: Koch snowflake (3 is animated)
5: Sierpinski triangle
6,7: Tree (10 blows in the wind)
*/
const EXERCISE: u8 = 10;
const ITERATIONS: u8 = 4;

const START: [f64; 2] = [100.0, 450.0];
const SHAPE_SIZE: f64 = 500.0;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {
	gl: GlGraphics,
	context: Context,
	koch_lines: Vec<KochLine>,
	line_index: usize,
	triangles: Vec<Triangle>,
	branches: Vec<Branch>,
	rotate_noise: RotatePoint<Perlin>,
}

impl App {
	// Exercise 1
	fn spiral_pattern(&mut self, x0: f64, y0: f64, length: f64, angle: f64) {
		if length > 1.0 {
			let x1 = x0 + length * angle.cos();
			let y1 = y0 + length * angle.sin();
			line_from_to(BLACK, 1.0, [x0, y0], [x1, y1], self.context.transform, &mut self.gl);
			self.spiral_pattern(x1, y1, length * 0.99, angle + 0.25);
		}
	}

	// Exercise 2 & 3
	fn generate_koch_lines(&mut self) {
		let mut new_lines: Vec<KochLine> = Vec::new();
		for line in &self.koch_lines {
			// Points
			let a: Vec2d = line.start;
			let b: Vec2d = lerp(line.start, line.end, 1.0 / 3.0);

			let mut c: Vec2d = math::sub(b, a); // Bring to origin
			let rot_matrix: math::Matrix2d = math::rotate_radians(PI / 3.0);
			c = math::transform_vec(rot_matrix, c); // Rotate 60 degrees
			c = math::add(b, c);

			let d: Vec2d = lerp(line.start, line.end, 2.0 / 3.0);
			let e: Vec2d = line.end;

			// Add lines between all points
			new_lines.push(KochLine {start: a, end: b});
			new_lines.push(KochLine {start: b, end: c});
			new_lines.push(KochLine {start: c, end: d});
			new_lines.push(KochLine {start: d, end: e});
		}
		self.koch_lines = new_lines;
	}

	fn draw_koch_line(&mut self, start: Vec2d, rotation: f64, max_index: usize) {
		let transform: math::Matrix2d = math::rotate_radians(rotation);

		for i in 0..max_index {
			let mut color: Color = BLACK;
			if EXERCISE == 3 {
				let index_ratio = i as f32 / self.koch_lines.len() as f32;
				color = Color::hue_deg([1.0, 0.0, 0.0, 1.0], index_ratio * 360.0);
			}
			let from: Vec2d = math::add(start, math::transform_vec(transform, self.koch_lines[i].start));
			let to: Vec2d = math::add(start, math::transform_vec(transform, self.koch_lines[i].end));
			
			line_from_to(color, 1.0, from, to, self.context.transform, &mut self.gl);
		}
	}

	fn koch_snowflake(&mut self, max_index: usize) {
		self.draw_koch_line(START, 0.0, max_index); // Bottom
		self.draw_koch_line([START[0] + SHAPE_SIZE, START[1]], -PI * 2.0 / 3.0, max_index); // Right
		self.draw_koch_line([START[0] + SHAPE_SIZE / 2.0, START[1] - (PI / 3.0).tan() * SHAPE_SIZE / 2.0], PI * 2.0 / 3.0, max_index); // Left
	}

	// Exercise 5
	fn sierpinski_triangle(&mut self) {
		for triangle in &self.triangles {
			polygon(BLACK, triangle, self.context.transform, &mut self.gl);
		}
	}
	
	fn generate_sierpinski(&mut self, triangle: Triangle, iterations: u8) {
		if iterations >= ITERATIONS {
			self.triangles.push(triangle);
			return;
		}
		let bottom_left: Triangle = [triangle[0], [triangle[2][0], triangle[0][1]], lerp(triangle[0], triangle[2], 1.0 / 2.0)];
		let right_left: Triangle = [[triangle[2][0], triangle[0][1]], triangle[1], lerp(triangle[1], triangle[2], 1.0 / 2.0)];
		let top: Triangle = [lerp(triangle[0], triangle[2], 1.0 / 2.0), lerp(triangle[1], triangle[2], 1.0 / 2.0), triangle[2]];

		self.generate_sierpinski(bottom_left, iterations + 1);
		self.generate_sierpinski(right_left, iterations + 1);
		self.generate_sierpinski(top, iterations + 1);
	}

	// Exercise 6, 7 & 10
	fn tree(&mut self) {
		let mut delta_pos = [0.0, 0.0];
		for branch in &mut self.branches {
			let mut delta_angle = 0.0;
			let direction = math::sub(branch.end, branch.start);
			if EXERCISE == 10 {
				delta_angle = self.rotate_noise.get(direction);
			}

			let rotation: Matrix2d = math::rotate_radians(delta_angle * PI / 180.0);
			let new_direction = math::transform_vec(rotation, direction);
			branch.end = math::add(branch.start, new_direction);
			delta_pos = math::add(delta_pos, math::sub(new_direction, direction));

			line_from_to(BLACK, branch.thickness, 
				math::add(branch.start, delta_pos), math::add(branch.end, delta_pos), 
				self.context.transform, &mut self.gl
			);
		}
	}

	fn generate_tree(&mut self, branch: Line, length: f64, thickness: f64) {
		let end: Vec2d = math::add(branch.start, math::mul_scalar(branch.direction, length));
		self.branches.push(Branch { start: branch.start, end, thickness });
		
		if length > 2.0 {
			let angle = PI / 6.0;

			let rotation_left: Matrix2d = math::rotate_radians(-angle);
			let left_direction = math::transform_vec(rotation_left, branch.direction);
			
			let rotation_right: Matrix2d = math::rotate_radians(angle);
			let right_direction = math::transform_vec(rotation_right, branch.direction);
			
			// Branch to both left and right
			self.generate_tree(Line {start: end, direction: left_direction}, length * 0.66, thickness * 0.8);
			self.generate_tree(Line {start: end, direction: right_direction}, length * 0.66, thickness * 0.8);
		}
	}

	fn render(&mut self, args: &RenderArgs) {
		let center = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

		self.context = self.gl.draw_begin(args.viewport());
		clear(WHITE, &mut self.gl);
		
		match EXERCISE {
			1 => self.spiral_pattern(center.0, 0.0, 75.0, 0.0),
			2 => self.koch_snowflake(self.koch_lines.len()),
			3 => self.koch_snowflake(self.line_index),
			5 => self.sierpinski_triangle(),
			6 | 10 => self.tree(),
			_ => (),
		}

		self.gl.draw_end();
	}

	fn update(&mut self, _args: &UpdateArgs) {
		self.line_index = self.koch_lines.len().min(self.line_index + 1);
	}
}

struct Line {
	start: Vec2d,
	direction: Vec2d,
}

struct Branch {
	start: Vec2d,
	end: Vec2d,
	thickness: f64,
}

struct KochLine {
	start: Vec2d,
	end: Vec2d,
}

// Linear interpolation
fn lerp(v1: Vec2d, v2: Vec2d, scalar: f64) -> Vec2d {
	let mut v3 = math::sub(v2, v1);
	v3 = math::mul_scalar(v3, scalar);
	v3 = math::add(v3, v1);
	v3
}

fn main() {
	// OpenGL version
	let opengl = OpenGL::V3_2;
	
	// Create window
	let mut window: Window = WindowSettings::new("Fractals", [800, 600])
	.graphics_api(opengl)
	.exit_on_esc(true)
	.build()
	.unwrap();
	
	// Create OpenGL back-end
	let gl = GlGraphics::new(opengl);

	let base_koch_line = KochLine { start: [0.0, 0.0], end: [SHAPE_SIZE, 0.0] };

	// Noise functions for randomly rotating vectors
	let perlin = Perlin::new(4);
	let rotate_noise = RotatePoint::new(perlin).set_z_angle(5.0);
	
	let mut app = App {
		gl,
		context: Context::new(),
		koch_lines: vec![base_koch_line],
		line_index: 0,
		triangles: Vec::new(),
		rotate_noise,
		branches: Vec::new(),
	};

	let size = window.ctx.window().inner_size();
	// Generate fractals
	match EXERCISE {
		2 | 3 => {
			for _i in 0..ITERATIONS {
				app.generate_koch_lines();
			}
		},
		5 => app.generate_sierpinski([START, [START[0] + SHAPE_SIZE, START[1]], [START[0] + SHAPE_SIZE / 2.0, START[1] - (PI / 3.0).tan() * SHAPE_SIZE / 2.0]], 0),
		6 | 10 => app.generate_tree(Line {start: [size.width as f64 / 2.0, size.height as f64], direction: [0.0, -1.0]}, 200.0, 5.0),
		_ => (),
	}
	
	let mut events = Events::new(EventSettings::new().max_fps(15));
	// Event loop
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args);
		}
		if let Some(args) = e.update_args() {
			app.update(&args);
		}
	}
}
