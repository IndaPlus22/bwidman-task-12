extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;

fn update(args: &UpdateArgs) {

}

fn render(args: &RenderArgs, gl: &mut GlGraphics) {
    use graphics::*;

    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let screen_width = args.window_size[0];
    let screen_height = args.window_size[1];
    let square = rectangle::centered_square(screen_width / 2.0, screen_height / 2.0, 50.0);

    gl.draw(args.viewport(), |context, gl| {
        clear(WHITE, gl);

        rectangle(BLACK, square, context.transform, gl);
    });
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
    let mut gl = GlGraphics::new(opengl);
    
    let mut events = Events::new(EventSettings::new());
    // Event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render(&args, &mut gl);
        }
        if let Some(args) = e.update_args() {
            update(&args);
        }
    }
}
