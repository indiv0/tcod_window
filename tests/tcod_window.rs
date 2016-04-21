extern crate piston;
extern crate tcod;
extern crate tcod_window;

use piston::event_loop::Events;
use piston::window::{Size, WindowSettings};
use tcod::console::Console;
use tcod_window::TcodWindow;

#[test]
fn test_tcod_window() {
    let mut window = TcodWindow::new(
        WindowSettings::new(
           "My Application".to_owned(),
            Size {
                width: 100,
                height: 100,
            }
        )
    );
    let mut events = window.events();

    window.window.borrow_mut().print(0, 0, "Test print!");

    assert!(events.next(&mut window).is_some());
}
