extern crate fps_counter;
extern crate piston;
extern crate tcod;
extern crate tcod_window;

use std::cell::RefCell;
use std::rc::Rc;

use fps_counter::FPSCounter;
use piston::event_loop::{EventLoop, Events};
use piston::input::Event::{Render, Update};
use piston::window::{Size, WindowSettings};
use tcod::Console;
use tcod::console::{Renderer, Root};
use tcod_window::TcodWindow;

const WINDOW_TITLE: &'static str = "fps_counter";
const WINDOW_SIZE_HEIGHT: u32 = 50;
const WINDOW_SIZE_WIDTH: u32 = 50;

fn main() {
    let settings = WindowSettings::new(WINDOW_TITLE,
                                       Size {
                                           width: WINDOW_SIZE_WIDTH,
                                           height: WINDOW_SIZE_HEIGHT,
                                       })
                       .exit_on_esc(true);

    let root = Root::initializer()
                      .size(settings.get_size().width as i32,
                            settings.get_size().height as i32)
                      .title(settings.get_title())
                      .renderer(Renderer::SDL)
                      .init();
    let console = Rc::new(RefCell::new(root));

    let mut window = TcodWindow::with_console(console, settings);
    let mut events = window.events().ups(140).max_fps(10000);

    let mut fps_counter = FPSCounter::new();
    let mut fps = fps_counter.tick();

    while let Some(e) = events.next(&mut window) {
        match e {
            Render(_) => {
                fps = fps_counter.tick();
            },
            Update(_) => {
                let fps_string = format!("FPS: {:?}", fps);
                window.window.borrow_mut().print(0, 0, fps_string);
            },
            _ => {},
        }
    }
}
