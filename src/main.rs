extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::f64::consts::PI;

use piston::window::WindowSettings;
use piston::input::{RenderArgs, RenderEvent};
use piston::event_loop::*;
use graphics::*;
use graphics::types::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;

const EXERCISE: u8 = 2;
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {
	gl: GlGraphics,
	context: Context,
	koch_lines: Vec<KochLine>,
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

	// Exercise 2
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

	fn draw_koch_line(&mut self, start: Vec2d, rotation: f64) {
		let transform: math::Matrix2d = self.context.transform;
		for (i, line) in self.koch_lines.iter().enumerate() {
			let mut color: Color = BLACK;
			if EXERCISE == 3 {
				let index_ratio = i as f32 / self.koch_lines.len() as f32;
				color = [0.8, index_ratio, index_ratio, 1.0];
			}
			line_from_to(
				color,
				1.0,
				math::add(start, line.start),
				math::add(start, line.end),
				transform,
				&mut self.gl
			);
		}
	}

	fn koch_snowflake(&mut self) {
		self.draw_koch_line([50.0, 50.0], PI);
	}

	fn render(&mut self, args: &RenderArgs) {
		let center = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
		// let square = rectangle::centered_square(screen_width / 2.0, screen_height / 2.0, 50.0);

		self.context = self.gl.draw_begin(args.viewport());

		clear(WHITE, &mut self.gl);

		// rectangle(BLACK, square, self.context.transform, &mut self.gl);
		
		match EXERCISE {
			1 => self.spiral_pattern(center.0, 0.0, 75.0, 0.0),
			2 | 3 => self.koch_snowflake(),
			_ => (),
		}

		self.gl.draw_end();
	}
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

	let base_koch_line = KochLine { start: [0.0, 0.0], end: [300.0, 0.0] };
	
	let mut app = App {
		gl,
		context: Context::new(),
		koch_lines: vec![base_koch_line],
	};

	if EXERCISE == 2 || EXERCISE == 3 {
		const ITERATIONS: u8 = 3;
		for _i in 0..ITERATIONS {
			app.generate_koch_lines();
		}
	}
	
	let mut events = Events::new(EventSettings::new());
	// Event loop
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args);
		}
	}
}
