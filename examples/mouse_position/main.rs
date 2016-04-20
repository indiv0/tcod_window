extern crate piston;
extern crate tcod;
extern crate tcod_window;

use piston::event_loop::Events;
use piston::input::Event::Input;
use piston::input::Input::Move;
use piston::input::Motion::MouseCursor;
use piston::window::{Size, WindowSettings};
use tcod::Console;
use tcod_window::TcodWindow;

const WINDOW_TITLE: &'static str = "mouse_position";
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

    window.window.borrow_mut().print(0, 0, "Move your mouse over the window!");

    while let Some(e) = events.next(&mut window) {
        if let Input(input) = e {
            if let Move(motion) = input {
                if let MouseCursor(x, y) = motion {
                    let mouse_string = format!("Mouse position (window): {}, {}", x, y);
                    window.window
                          .borrow_mut()
                          .print(0, 1, mouse_string);
                }
            }
        }
    }
}
