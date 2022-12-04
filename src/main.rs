extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::f64::consts::PI;

use graphics::types::Vec2d;
use piston::window::WindowSettings;
use piston::input::{RenderArgs, RenderEvent};
use piston::event_loop::*;
use graphics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;

const EXERCISE: u8 = 1;
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {
	gl: GlGraphics,
	context: Context,
}

impl App {
	// In order: Exercise 1, 2, 3,..., 14

	fn spiral_pattern(&mut self, x0: f64, y0: f64, length: f64, angle: f64) {
		if length > 1.0 {
			let x1 = x0 + length * angle.cos();
			let y1 = y0 + length * angle.sin();
			line_from_to(BLACK, 1.0, [x0, y0], [x1, y1], self.context.transform, &mut self.gl);
			self.spiral_pattern(x1, y1, length * 0.99, angle + 0.25);
		}
	}

	fn render(&mut self, args: &RenderArgs) {
		let center = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
		// let square = rectangle::centered_square(screen_width / 2.0, screen_height / 2.0, 50.0);

		self.context = self.gl.draw_begin(args.viewport());

		clear(WHITE, &mut self.gl);

		// rectangle(BLACK, square, self.context.transform, &mut self.gl);
		
		match EXERCISE {
			1 => self.spiral_pattern(center.0, 0.0, 75.0, 0.0),
			_ => (),
		}

		self.gl.draw_end();
	}
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

	let mut app = App {
		gl,
		context: Context::new(),
	};
	
	let mut events = Events::new(EventSettings::new());
	// Event loop
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args);
		}
	}
}
