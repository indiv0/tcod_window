extern crate piston;
extern crate tcod;
extern crate tcod_window;

use piston::event_loop::Events;
use piston::window::{Size, WindowSettings};
use tcod::Console;
use tcod_window::TcodWindow;

const WINDOW_TITLE: &'static str = "hello_world";
const WINDOW_SIZE_HEIGHT: u32 = 61;
const WINDOW_SIZE_WIDTH: u32 = 99;

fn main() {
    let settings = WindowSettings::new(WINDOW_TITLE,
                                       Size {
                                           width: WINDOW_SIZE_WIDTH,
                                           height: WINDOW_SIZE_HEIGHT,
                                       })
                       .exit_on_esc(true);

    let mut window = TcodWindow::new(settings);
    let mut events = window.events();

    window.window.borrow_mut().print(0, 0, "Hello, world!");

    while let Some(_) = events.next(&mut window) {}
}
