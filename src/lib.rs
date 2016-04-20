#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! A TCOD back-end for the Piston game engine.

extern crate input;
#[macro_use]
extern crate tcod;
extern crate window;

use std::cell::RefCell;
use std::rc::Rc;

use input::{Input, MouseButton};
use input::keyboard::Key as PistonKey;
use tcod::input::{Key as TcodKey, KeyCode, Mouse};
use tcod::Console;
use tcod::console::Root;
use window::{AdvancedWindow, BuildFromWindowSettings, Size, Window, WindowSettings};

pub struct TcodWindow {
    pub window: Rc<RefCell<Root>>,
    title: String,
    should_close: bool,
    mouse_relative: Option<(f64, f64)>,
    mouse_state_prev: Mouse,
    exit_on_esc: bool,
}

impl TcodWindow {
    pub fn new(settings: WindowSettings) -> Self {
        let console = Root::initializer()
                          .size(settings.get_size().width as i32,
                                settings.get_size().height as i32)
                          .title(settings.get_title())
                          .init();
        let console = Rc::new(RefCell::new(console));

        Self::with_console(console, settings)
    }

    pub fn with_console(console: Rc<RefCell<Root>>, settings: WindowSettings) -> Self {
        TcodWindow {
            window: console,
            title: settings.get_title(),
            should_close: false,
            mouse_relative: None,
            mouse_state_prev: Mouse::default(),
            exit_on_esc: settings.get_exit_on_esc(),
        }
    }

    fn poll_event(&mut self) -> Option<Input> {
        use input::{Button, Motion};
        use input::Input::{Move, Press, Release};
        use tcod::input::{ANY, KEY_PRESS, KEY_RELEASE, MOUSE_MOVE, MOUSE_PRESS, MOUSE_RELEASE,
                          check_for_event};
        use tcod::input::Event::{Key, Mouse};

        if let Some((x, y)) = self.mouse_relative {
            self.mouse_relative = None;
            return Some(Move(Motion::MouseRelative(x, y)));
        }

        match check_for_event(ANY) {
            Some((KEY_PRESS, Key(ref key_state))) => {
                if self.exit_on_esc && key_state.code == KeyCode::Escape {
                    self.should_close = true;
                    None
                } else {
                    Some(Press(Button::Keyboard(tcod_map_key(*key_state))))
                }
            },
            Some((KEY_RELEASE, Key(ref key_state))) => {
                Some(Release(Button::Keyboard(tcod_map_key(*key_state))))
            },
            Some((MOUSE_PRESS, Mouse(ref mouse_state))) => {
                let button = tcod_map_mouse(self.mouse_state_prev, mouse_state);
                self.mouse_state_prev = *mouse_state;
                Some(Press(Button::Mouse(button)))
            },
            Some((MOUSE_RELEASE, Mouse(ref mouse_state))) => {
                let button = tcod_map_mouse(self.mouse_state_prev, mouse_state);
                self.mouse_state_prev = *mouse_state;
                Some(Release(Button::Mouse(button)))
            },
            Some((MOUSE_MOVE, Mouse(ref mouse_state))) => {
                self.mouse_relative = Some(((mouse_state.x - self.mouse_state_prev.x) as f64,
                                            (mouse_state.y - self.mouse_state_prev.y) as f64));
                self.mouse_state_prev = *mouse_state;
                Some(Move(Motion::MouseCursor(mouse_state.x as f64, mouse_state.y as f64)))
            },
            _ => None,
        }
    }
}

impl BuildFromWindowSettings for TcodWindow {
    fn build_from_window_settings(settings: WindowSettings) -> Result<Self, String> {
        Ok(TcodWindow::new(settings))
    }
}

impl Window for TcodWindow {
    type Event = Input;

    fn should_close(&self) -> bool {
        self.should_close
    }
    fn set_should_close(&mut self, value: bool) {
        self.should_close = value;
    }
    fn swap_buffers(&mut self) {
        self.window.borrow_mut().flush();
    }
    fn size(&self) -> Size {
        let window = self.window.borrow();
        Size {
            width: window.width() as u32,
            height: window.height() as u32,
        }
    }
    fn poll_event(&mut self) -> Option<Input> {
        self.poll_event()
    }
    fn draw_size(&self) -> Size {
        let window = self.window.borrow();
        Size {
            width: window.width() as u32,
            height: window.height() as u32,
        }
    }
}

impl AdvancedWindow for TcodWindow {
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn set_title(&mut self, value: String) {
        self.window.borrow_mut().set_window_title(&value);
        self.title = value
    }
    fn get_exit_on_esc(&self) -> bool {
        self.exit_on_esc
    }
    fn set_exit_on_esc(&mut self, value: bool) {
        self.exit_on_esc = value;
    }
    fn set_capture_cursor(&mut self, _value: bool) {}
}

pub fn tcod_map_key(key: TcodKey) -> PistonKey {
    match key.code {
        KeyCode::NoKey => PistonKey::Unknown,
        KeyCode::Escape => PistonKey::Escape,
        KeyCode::Backspace => PistonKey::Backspace,
        KeyCode::Tab => PistonKey::Tab,
        KeyCode::Enter => PistonKey::Return,
        KeyCode::Shift => PistonKey::LShift,
        KeyCode::Control => PistonKey::LCtrl,
        KeyCode::Alt => PistonKey::LAlt,
        KeyCode::Pause => PistonKey::Pause,
        KeyCode::CapsLock => PistonKey::CapsLock,
        KeyCode::PageUp => PistonKey::PageUp,
        KeyCode::PageDown => PistonKey::PageDown,
        KeyCode::End => PistonKey::End,
        KeyCode::Home => PistonKey::Home,
        KeyCode::Up => PistonKey::Up,
        KeyCode::Left => PistonKey::Left,
        KeyCode::Right => PistonKey::Right,
        KeyCode::Down => PistonKey::Down,
        KeyCode::PrintScreen => PistonKey::PrintScreen,
        KeyCode::Insert => PistonKey::Insert,
        KeyCode::Delete => PistonKey::Delete,
        KeyCode::LeftWin => PistonKey::LGui,
        KeyCode::RightWin => PistonKey::RGui,
        KeyCode::Apps => PistonKey::Application,
        // The numbers on the numeric keypad
        KeyCode::NumPad0 => PistonKey::NumPad0,
        KeyCode::NumPad1 => PistonKey::NumPad1,
        KeyCode::NumPad2 => PistonKey::NumPad2,
        KeyCode::NumPad3 => PistonKey::NumPad3,
        KeyCode::NumPad4 => PistonKey::NumPad4,
        KeyCode::NumPad5 => PistonKey::NumPad5,
        KeyCode::NumPad6 => PistonKey::NumPad6,
        KeyCode::NumPad7 => PistonKey::NumPad7,
        KeyCode::NumPad8 => PistonKey::NumPad8,
        KeyCode::NumPad9 => PistonKey::NumPad9,
        KeyCode::NumPadAdd => PistonKey::NumPadPlus,
        KeyCode::NumPadSubtract => PistonKey::NumPadMinus,
        KeyCode::NumPadDivide => PistonKey::NumPadDivide,
        KeyCode::NumPadMultiply => PistonKey::NumPadMultiply,
        KeyCode::NumPadDecimal => PistonKey::NumPadDecimal,
        KeyCode::NumPadEnter => PistonKey::NumPadEnter,
        KeyCode::F1 => PistonKey::F1,
        KeyCode::F2 => PistonKey::F2,
        KeyCode::F3 => PistonKey::F3,
        KeyCode::F4 => PistonKey::F4,
        KeyCode::F5 => PistonKey::F5,
        KeyCode::F6 => PistonKey::F6,
        KeyCode::F7 => PistonKey::F7,
        KeyCode::F8 => PistonKey::F8,
        KeyCode::F9 => PistonKey::F9,
        KeyCode::F10 => PistonKey::F10,
        KeyCode::F11 => PistonKey::F11,
        KeyCode::F12 => PistonKey::F12,
        KeyCode::NumLock => PistonKey::NumLockClear,
        KeyCode::ScrollLock => PistonKey::ScrollLock,
        KeyCode::Spacebar => PistonKey::Space,
        // Characters
        _ => {
            let key_as_u32 = match key.printable {
                // We need to convert the uppercase characters to be lowercase.
                key @ 'A' ... 'Z' => key.to_lowercase().next().unwrap() as u32,
                key => key as u32,
            };

            key_as_u32.into()
        },
    }
}

#[cfg_attr(feature = "clippy", allow(if_not_else))]
pub fn tcod_map_mouse(prev_state: Mouse, state: &Mouse) -> MouseButton {
    if prev_state.lbutton != state.lbutton {
        MouseButton::Left
    } else if prev_state.rbutton != state.rbutton {
        MouseButton::Right
    } else if prev_state.mbutton != state.mbutton {
        MouseButton::Middle
    } else {
        MouseButton::Unknown
    }
}
