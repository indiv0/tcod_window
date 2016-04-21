// Copyright 2015-2016 Nikita Pekin and the tcod_window contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! This crate provides a TCOD back-end for the Piston game engine.
//!
//! # Example
//!
//! The following example shows a quick example of the basic functionality of
//! `TcodWindow`.
//!
//! ```
//! extern crate piston;
//! extern crate tcod_window;
//!
//! use piston::window::{Size, WindowSettings};
//! use tcod_window::TcodWindow;
//!
//! fn main() {
//!     let mut window = TcodWindow::new(
//!         WindowSettings::new(
//!             "My Application".to_owned(),
//!             Size {
//!                 width: 100,
//!                 height: 100,
//!             }
//!         )
//!     );
//! }
//! ```

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

/// A window implemented by a TCOD back-end.
pub struct TcodWindow {
    /// TCOD `Root` window used for rendering.
    pub window: Rc<RefCell<Root>>,
    title: String,
    should_close: bool,
    mouse_relative: Option<(f64, f64)>,
    mouse_state_prev: Mouse,
    exit_on_esc: bool,
}

impl TcodWindow {
    /// Create a new game window from the provided `WindowSettings`.
    ///
    /// Internally, it initializes a new TCOD `Root` with the size and title
    /// specified in the `WindowSettings`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate piston;
    /// # extern crate tcod_window;
    /// #
    /// use piston::window::{Size, WindowSettings};
    /// use tcod_window::TcodWindow;
    ///
    /// # fn main() {
    /// let mut window = TcodWindow::new(
    ///     WindowSettings::new(
    ///         "My Application".to_owned(),
    ///         Size {
    ///             width: 100,
    ///             height: 100,
    ///         }
    ///     )
    /// );
    /// # }
    /// ```
    pub fn new(settings: WindowSettings) -> Self {
        let console = Root::initializer()
                          .size(settings.get_size().width as i32,
                                settings.get_size().height as i32)
                          .title(settings.get_title())
                          .init();
        let console = Rc::new(RefCell::new(console));

        Self::with_console(console, settings)
    }

    /// Create a new game window from an existing TCOD `Root` console wrapped as
    /// an `Rc<RefCell<Root>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate piston;
    /// # extern crate tcod;
    /// # extern crate tcod_window;
    /// #
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// #
    /// use piston::window::{Size, WindowSettings};
    /// use tcod::console::Root;
    /// use tcod_window::TcodWindow;
    ///
    /// # fn main() {
    /// let settings = WindowSettings::new(
    ///     "My Application".to_owned(),
    ///     Size {
    ///         width: 100,
    ///         height: 100,
    ///     }
    /// );
    /// let root = Root::initializer()
    ///                 .size(settings.get_size().width as i32,
    ///                       settings.get_size().height as i32)
    ///                 .title(settings.get_title())
    ///                 .init();
    /// let console = Rc::new(RefCell::new(root));
    ///
    /// let mut window = TcodWindow::with_console(console, settings);
    /// # }
    /// ```
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

/// Maps a TCOD key to a piston-input key.
///
/// # Examples
///
/// ```
/// # extern crate piston;
/// # extern crate tcod;
/// # extern crate tcod_window;
/// #
/// use piston::input::Key as PistonKey;
/// use tcod::input::{Key, KeyCode};
/// use tcod_window::tcod_map_key;
///
/// # fn main() {
/// let tcod_key = Key {
///     code: KeyCode::Char,
///     printable: 'A',
///     ..Key::default()
/// };
///
/// assert_eq!(tcod_map_key(tcod_key), PistonKey::A);
/// # }
/// ```
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

/// Maps a TCOD mouse state change to a piston-input button.
///
/// # Examples
///
/// ```
/// # extern crate piston;
/// # extern crate tcod;
/// # extern crate tcod_window;
/// #
/// use piston::input::mouse::MouseButton;
/// use tcod::input::Mouse;
/// use tcod_window::tcod_map_mouse;
///
/// # fn main() {
/// let prev_state = Mouse {
///     lbutton: false,
///     ..Mouse::default()
/// };
/// let state = Mouse {
///     lbutton: true,
///     ..Mouse::default()
/// };
///
/// assert_eq!(tcod_map_mouse(prev_state, &state), MouseButton::Left);
/// # }
/// ```
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

#[cfg(test)]
mod tests {
    extern crate piston;

    use self::piston::window::{Size, WindowSettings};

    use super::TcodWindow;
    use super::tcod::input::{Key, KeyCode};

    fn tcod_key_from_keycode(key_code: KeyCode) -> Key {
        Key {
            code: key_code,
            ..Key::default()
        }
    }

    fn tcod_key_from_char(c: char) -> Key {
        Key {
            code: KeyCode::Char,
            printable: c,
            ..Key::default()
        }
    }

    #[test]
    fn test_new() {
        let _ = TcodWindow::new(
            WindowSettings::new(
               "My Application".to_owned(),
                Size {
                    width: 100,
                    height: 100,
                }
            )
        );
    }

    #[test]
    fn test_from_console() {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::tcod::console::Root;

        let settings = WindowSettings::new(
            "My Application".to_owned(),
            Size {
                width: 100,
                height: 100,
            }
        );
        let root = Root::initializer()
                        .size(settings.get_size().width as i32,
                              settings.get_size().height as i32)
                        .title(settings.get_title())
                        .init();
        let console = Rc::new(RefCell::new(root));

        let _ = TcodWindow::with_console(console, settings);
    }

    #[test]
    fn test_build_from_window_settings() {
        let settings = WindowSettings::new(
               "My Application".to_owned(),
                Size {
                    width: 100,
                    height: 100,
                }
            )
            .exit_on_esc(true);

        let _: TcodWindow = settings.build()
            .expect("Failed to build window.");
    }

    #[test]
    fn test_window() {
        use self::piston::window::Window;

        let mut window = TcodWindow::new(
            WindowSettings::new(
               "My Application".to_owned(),
                Size {
                    width: 100,
                    height: 100,
                }
            )
        );

        assert_eq!(window.should_close(), false);
        window.set_should_close(true);
        assert_eq!(window.should_close(), true);

        window.swap_buffers();

        let size = window.size();
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 100);

        assert_eq!(window.poll_event(), None);

        let draw_size = window.draw_size();
        assert_eq!(draw_size.width, 100);
        assert_eq!(draw_size.height, 100);
    }

    #[test]
    fn test_advanced_window() {
        use self::piston::window::AdvancedWindow;

        let mut window = TcodWindow::new(
            WindowSettings::new(
               "My Application".to_owned(),
                Size {
                    width: 100,
                    height: 100,
                }
            )
        );

        assert_eq!(window.get_title(), "My Application".to_owned());
        window.set_title("some other name".to_owned());
        assert_eq!(window.get_title(), "some other name".to_owned());

        assert_eq!(window.get_exit_on_esc(), false);
        window.set_exit_on_esc(true);
        assert_eq!(window.get_exit_on_esc(), true);

        window.set_capture_cursor(true);
    }

    #[test]
    #[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
    fn test_tcod_map_key() {
        use self::piston::input::Key as PistonKey;

        use super::tcod_map_key;

        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NoKey)), PistonKey::Unknown);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Escape)), PistonKey::Escape);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Backspace)), PistonKey::Backspace);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Tab)), PistonKey::Tab);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Enter)), PistonKey::Return);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Shift)), PistonKey::LShift);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Control)), PistonKey::LCtrl);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Alt)), PistonKey::LAlt);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Pause)), PistonKey::Pause);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::CapsLock)), PistonKey::CapsLock);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::PageUp)), PistonKey::PageUp);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::PageDown)), PistonKey::PageDown);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::End)), PistonKey::End);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Home)), PistonKey::Home);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Up)), PistonKey::Up);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Left)), PistonKey::Left);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Right)), PistonKey::Right);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Down)), PistonKey::Down);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::PrintScreen)), PistonKey::PrintScreen);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Insert)), PistonKey::Insert);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Delete)), PistonKey::Delete);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::LeftWin)), PistonKey::LGui);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::RightWin)), PistonKey::RGui);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Apps)), PistonKey::Application);
        // The numbers on the numeric keypad
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad0)), PistonKey::NumPad0);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad1)), PistonKey::NumPad1);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad2)), PistonKey::NumPad2);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad3)), PistonKey::NumPad3);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad4)), PistonKey::NumPad4);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad5)), PistonKey::NumPad5);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad6)), PistonKey::NumPad6);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad7)), PistonKey::NumPad7);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad8)), PistonKey::NumPad8);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPad9)), PistonKey::NumPad9);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadAdd)), PistonKey::NumPadPlus);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadSubtract)), PistonKey::NumPadMinus);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadDivide)), PistonKey::NumPadDivide);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadMultiply)), PistonKey::NumPadMultiply);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadDecimal)), PistonKey::NumPadDecimal);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumPadEnter)), PistonKey::NumPadEnter);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F1)), PistonKey::F1);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F2)), PistonKey::F2);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F3)), PistonKey::F3);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F4)), PistonKey::F4);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F5)), PistonKey::F5);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F6)), PistonKey::F6);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F7)), PistonKey::F7);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F8)), PistonKey::F8);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F9)), PistonKey::F9);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F10)), PistonKey::F10);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F11)), PistonKey::F11);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::F12)), PistonKey::F12);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::NumLock)), PistonKey::NumLockClear);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::ScrollLock)), PistonKey::ScrollLock);
        assert_eq!(tcod_map_key(tcod_key_from_keycode(KeyCode::Spacebar)), PistonKey::Space);
        // Uppercase characters
        assert_eq!(tcod_map_key(tcod_key_from_char('A')), PistonKey::A);
        assert_eq!(tcod_map_key(tcod_key_from_char('B')), PistonKey::B);
        assert_eq!(tcod_map_key(tcod_key_from_char('C')), PistonKey::C);
        assert_eq!(tcod_map_key(tcod_key_from_char('D')), PistonKey::D);
        assert_eq!(tcod_map_key(tcod_key_from_char('E')), PistonKey::E);
        assert_eq!(tcod_map_key(tcod_key_from_char('F')), PistonKey::F);
        assert_eq!(tcod_map_key(tcod_key_from_char('G')), PistonKey::G);
        assert_eq!(tcod_map_key(tcod_key_from_char('H')), PistonKey::H);
        assert_eq!(tcod_map_key(tcod_key_from_char('I')), PistonKey::I);
        assert_eq!(tcod_map_key(tcod_key_from_char('J')), PistonKey::J);
        assert_eq!(tcod_map_key(tcod_key_from_char('K')), PistonKey::K);
        assert_eq!(tcod_map_key(tcod_key_from_char('L')), PistonKey::L);
        assert_eq!(tcod_map_key(tcod_key_from_char('M')), PistonKey::M);
        assert_eq!(tcod_map_key(tcod_key_from_char('N')), PistonKey::N);
        assert_eq!(tcod_map_key(tcod_key_from_char('O')), PistonKey::O);
        assert_eq!(tcod_map_key(tcod_key_from_char('P')), PistonKey::P);
        assert_eq!(tcod_map_key(tcod_key_from_char('Q')), PistonKey::Q);
        assert_eq!(tcod_map_key(tcod_key_from_char('R')), PistonKey::R);
        assert_eq!(tcod_map_key(tcod_key_from_char('S')), PistonKey::S);
        assert_eq!(tcod_map_key(tcod_key_from_char('T')), PistonKey::T);
        assert_eq!(tcod_map_key(tcod_key_from_char('U')), PistonKey::U);
        assert_eq!(tcod_map_key(tcod_key_from_char('V')), PistonKey::V);
        assert_eq!(tcod_map_key(tcod_key_from_char('W')), PistonKey::W);
        assert_eq!(tcod_map_key(tcod_key_from_char('X')), PistonKey::X);
        assert_eq!(tcod_map_key(tcod_key_from_char('Y')), PistonKey::Y);
        assert_eq!(tcod_map_key(tcod_key_from_char('Z')), PistonKey::Z);
        // Lowercase characters
        assert_eq!(tcod_map_key(tcod_key_from_char('a')), PistonKey::A);
        assert_eq!(tcod_map_key(tcod_key_from_char('b')), PistonKey::B);
        assert_eq!(tcod_map_key(tcod_key_from_char('c')), PistonKey::C);
        assert_eq!(tcod_map_key(tcod_key_from_char('d')), PistonKey::D);
        assert_eq!(tcod_map_key(tcod_key_from_char('e')), PistonKey::E);
        assert_eq!(tcod_map_key(tcod_key_from_char('f')), PistonKey::F);
        assert_eq!(tcod_map_key(tcod_key_from_char('g')), PistonKey::G);
        assert_eq!(tcod_map_key(tcod_key_from_char('h')), PistonKey::H);
        assert_eq!(tcod_map_key(tcod_key_from_char('i')), PistonKey::I);
        assert_eq!(tcod_map_key(tcod_key_from_char('j')), PistonKey::J);
        assert_eq!(tcod_map_key(tcod_key_from_char('k')), PistonKey::K);
        assert_eq!(tcod_map_key(tcod_key_from_char('l')), PistonKey::L);
        assert_eq!(tcod_map_key(tcod_key_from_char('m')), PistonKey::M);
        assert_eq!(tcod_map_key(tcod_key_from_char('n')), PistonKey::N);
        assert_eq!(tcod_map_key(tcod_key_from_char('o')), PistonKey::O);
        assert_eq!(tcod_map_key(tcod_key_from_char('p')), PistonKey::P);
        assert_eq!(tcod_map_key(tcod_key_from_char('q')), PistonKey::Q);
        assert_eq!(tcod_map_key(tcod_key_from_char('r')), PistonKey::R);
        assert_eq!(tcod_map_key(tcod_key_from_char('s')), PistonKey::S);
        assert_eq!(tcod_map_key(tcod_key_from_char('t')), PistonKey::T);
        assert_eq!(tcod_map_key(tcod_key_from_char('u')), PistonKey::U);
        assert_eq!(tcod_map_key(tcod_key_from_char('v')), PistonKey::V);
        assert_eq!(tcod_map_key(tcod_key_from_char('w')), PistonKey::W);
        assert_eq!(tcod_map_key(tcod_key_from_char('x')), PistonKey::X);
        assert_eq!(tcod_map_key(tcod_key_from_char('y')), PistonKey::Y);
        assert_eq!(tcod_map_key(tcod_key_from_char('z')), PistonKey::Z);
        // Digits
        assert_eq!(tcod_map_key(tcod_key_from_char('0')), PistonKey::D0);
        assert_eq!(tcod_map_key(tcod_key_from_char('1')), PistonKey::D1);
        assert_eq!(tcod_map_key(tcod_key_from_char('2')), PistonKey::D2);
        assert_eq!(tcod_map_key(tcod_key_from_char('3')), PistonKey::D3);
        assert_eq!(tcod_map_key(tcod_key_from_char('4')), PistonKey::D4);
        assert_eq!(tcod_map_key(tcod_key_from_char('5')), PistonKey::D5);
        assert_eq!(tcod_map_key(tcod_key_from_char('6')), PistonKey::D6);
        assert_eq!(tcod_map_key(tcod_key_from_char('7')), PistonKey::D7);
        assert_eq!(tcod_map_key(tcod_key_from_char('8')), PistonKey::D8);
        assert_eq!(tcod_map_key(tcod_key_from_char('9')), PistonKey::D9);
        // Shift + Digits
        assert_eq!(tcod_map_key(tcod_key_from_char('!')), PistonKey::Exclaim);
        assert_eq!(tcod_map_key(tcod_key_from_char('@')), PistonKey::At);
        assert_eq!(tcod_map_key(tcod_key_from_char('#')), PistonKey::Hash);
        assert_eq!(tcod_map_key(tcod_key_from_char('$')), PistonKey::Dollar);
        assert_eq!(tcod_map_key(tcod_key_from_char('%')), PistonKey::Percent);
        assert_eq!(tcod_map_key(tcod_key_from_char('^')), PistonKey::Caret);
        assert_eq!(tcod_map_key(tcod_key_from_char('&')), PistonKey::Ampersand);
        assert_eq!(tcod_map_key(tcod_key_from_char('*')), PistonKey::Asterisk);
        assert_eq!(tcod_map_key(tcod_key_from_char('(')), PistonKey::LeftParen);
        assert_eq!(tcod_map_key(tcod_key_from_char(')')), PistonKey::RightParen);
    }

    #[test]
    fn test_tcod_map_mouse() {
        use self::piston::input::mouse::MouseButton;

        use super::tcod::input::Mouse;
        use super::tcod_map_mouse;

        let prev_state = Mouse {
            lbutton: false,
           ..Mouse::default()
        };
        let state = Mouse {
            lbutton: true,
            ..Mouse::default()
        };

        assert_eq!(tcod_map_mouse(prev_state, &state), MouseButton::Left);

        let prev_state = Mouse {
            rbutton: false,
           ..Mouse::default()
        };
        let state = Mouse {
            rbutton: true,
            ..Mouse::default()
        };

        assert_eq!(tcod_map_mouse(prev_state, &state), MouseButton::Right);

        let prev_state = Mouse {
            mbutton: false,
           ..Mouse::default()
        };
        let state = Mouse {
            mbutton: true,
            ..Mouse::default()
        };

        assert_eq!(tcod_map_mouse(prev_state, &state), MouseButton::Middle);

        let prev_state = Mouse {
           ..Mouse::default()
        };
        let state = Mouse {
            ..Mouse::default()
        };

        assert_eq!(tcod_map_mouse(prev_state, &state), MouseButton::Unknown);
    }
}
