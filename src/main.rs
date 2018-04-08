extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod pong;

use piston::{window::WindowSettings, event_loop::{EventSettings, Events}};
use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use pong::Pong;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pong", [512, 342])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut pong = Pong::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            pong.render(&r);
        }

        if let Some(u) = e.update_args() {
            pong.update(&u);
        }

        if let Some(b) = e.press_args() {
            pong.press(&b);
        }

        if let Some(b) = e.release_args() {
            pong.release(&b);
        }
    }
}
