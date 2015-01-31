#![feature(core)]

//! A TCOD back-end for the Piston game engine.

extern crate input;
#[macro_use]
extern crate quack;
extern crate tcod;
extern crate window;

use input::{ keyboard, Button, MouseButton, Input, Motion };
use window::{
    WindowSettings,
    ShouldClose, Size, PollEvent, SwapBuffers,
    CaptureCursor, DrawSize, Title, ExitOnEsc
};
use quack::Set;
use tcod::Console;

pub struct TcodWindow {
    pub window: tcod::Console,
    title: String,
    should_close: bool,
    mouse_relative: Option<(f64, f64)>,
    mouse_state_prev: tcod::MouseState,
    exit_on_esc: bool,
}

impl TcodWindow {
    pub fn new(settings: WindowSettings) -> TcodWindow {
        let window = Console::init_root(
            settings.size[0] as i32,
            settings.size[1] as i32,
            &*settings.title,
            false
        );

        TcodWindow {
            window: window,
            title: settings.title,
            should_close: false,
            mouse_relative: None,
            // TODO: determine if there's a better way to initialize the
            // MouseState.
            mouse_state_prev: tcod::MouseState {
                x: 0is,
                y: 0is,
                dx: 0is,
                dy: 0is,
                cx: 0is,
                cy: 0is,
                dcx: 0is,
                dcy: 0is,
                lbutton: false,
                rbutton: false,
                mbutton: false,
                lbutton_pressed: false,
                rbutton_pressed: false,
                mbutton_pressed: false,
                wheel_up: false,
                wheel_down: false,
            },
            exit_on_esc: settings.exit_on_esc,
        }
    }

    fn poll_event(&mut self) -> Option<Input> {
        use tcod::KeyCode;
        use tcod::Key::Special;
        use tcod::system::Event;

        match self.mouse_relative {
            Some((x, y)) => {
                self.mouse_relative = None;
                return Some(Input::Move(Motion::MouseRelative(x, y)));
            }
            None => {}
        }
        let (flags, event) = tcod::system::check_for_event(tcod::ANY);
        match (flags, event) {
            (tcod::KEY_PRESS, Event::Key(ref key_state)) => {
                if self.exit_on_esc
                && key_state.key == Special(KeyCode::Escape) {
                    self.should_close = true;
                } else {
                    return Some(Input::Press(Button::Keyboard(tcod_map_key(key_state.key))));
                }
            },
            (tcod::KEY_RELEASE, Event::Key(ref key_state)) => {
                return Some(Input::Release(Button::Keyboard(tcod_map_key(key_state.key))));
            },
            (tcod::MOUSE_PRESS, Event::Mouse(ref mouse_state)) => {
                let button = tcod_map_mouse(self.mouse_state_prev, mouse_state);
                self.mouse_state_prev = *mouse_state.clone();
                return Some(Input::Press(Button::Mouse(button)));
            },
            (tcod::MOUSE_RELEASE, Event::Mouse(ref mouse_state)) => {
                let button = tcod_map_mouse(self.mouse_state_prev, mouse_state);
                self.mouse_state_prev = *mouse_state.clone();
                return Some(Input::Release(Button::Mouse(button)));
            },
            (tcod::MOUSE_MOVE, Event::Mouse(ref mouse_state)) => {
                // TODO: figure out why mouse_state.dx and mouse_state.dy
                // don't work.
                self.mouse_relative = Some((
                    (mouse_state.x - self.mouse_state_prev.x) as f64,
                    (mouse_state.y - self.mouse_state_prev.y) as f64
                ));
                self.mouse_state_prev = *mouse_state.clone();
                return Some(Input::Move(Motion::MouseCursor(mouse_state.x as f64, mouse_state.y as f64)));
            },
            (_, _) => {}
        }
        None
    }
}

impl Drop for TcodWindow {
    fn drop(&mut self) {
        self.set_mut(CaptureCursor(false));
    }
}

quack! {
    _obj: TcodWindow[]
    get:
        fn () -> ShouldClose { ShouldClose(_obj.should_close) }
        fn () -> Size {
            let (w, h) = (_obj.window.width(), _obj.window.height());
            Size([w as u32, h as u32])
        }
        fn () -> DrawSize {
            //let (w, h) = _obj.window.get_drawable_size();
            let (w, h) = (_obj.window.width(), _obj.window.height());
            DrawSize([w as u32, h as u32])
        }
        // TODO: Remove the .clone()
        fn () -> Title { Title(_obj.title.clone()) }
        fn () -> ExitOnEsc { ExitOnEsc(_obj.exit_on_esc) }
    set:
        fn (val: CaptureCursor) {
            //sdl2::mouse::set_relative_mouse_mode(val.0)
        }
        fn (val: ShouldClose) { _obj.should_close = val.0 }
        fn (val: Title) {
            Console::set_window_title(&*val.0);
            _obj.title = val.0
        }
        fn (val: ExitOnEsc) { _obj.exit_on_esc = val.0 }
    action:
        fn (__: SwapBuffers) -> () {
            //_obj.window.gl_swap_window()
            Console::flush()
        }
        fn (__: PollEvent) -> Option<Input> { _obj.poll_event() }
}

pub fn tcod_map_key(key: tcod::Key) -> keyboard::Key {
    use input::keyboard::Key;
    use std::num::FromPrimitive;
    match key {
        tcod::Key::Printable(keycode) => FromPrimitive::from_u64(keycode as u64).unwrap(),
        tcod::Key::Special(keycode) => {
            match keycode {
                tcod::KeyCode::NoKey => Key::Unknown,
                tcod::KeyCode::Escape => Key::Escape,
                tcod::KeyCode::Backspace => Key::Backspace,
                tcod::KeyCode::Tab => Key::Tab,
                tcod::KeyCode::Enter => Key::Return,
                tcod::KeyCode::Shift => Key::LShift, // TODO: find RShift
                tcod::KeyCode::Control => Key::LCtrl,
                tcod::KeyCode::Alt => Key::LAlt,
                tcod::KeyCode::Pause => Key::Pause,
                tcod::KeyCode::Capslock => Key::CapsLock,
                tcod::KeyCode::Pageup => Key::PageUp,
                tcod::KeyCode::Pagedown => Key::PageDown,
                tcod::KeyCode::End => Key::End,
                tcod::KeyCode::Home => Key::Home,
                tcod::KeyCode::Up => Key::Up,
                tcod::KeyCode::Left => Key::Left,
                tcod::KeyCode::Right => Key::Right,
                tcod::KeyCode::Down => Key::Down,
                tcod::KeyCode::PrintScreen => Key::PrintScreen,
                tcod::KeyCode::Insert => Key::Insert,
                tcod::KeyCode::Delete => Key::Delete,
                tcod::KeyCode::LeftWin => Key::LGui,
                tcod::KeyCode::RightWin => Key::RGui,
                tcod::KeyCode::Apps => Key::Application,
                // The numbers on the alphanum section of the keyboard
                tcod::KeyCode::Number0 => Key::D0,
                tcod::KeyCode::Number1 => Key::D1,
                tcod::KeyCode::Number2 => Key::D2,
                tcod::KeyCode::Number3 => Key::D3,
                tcod::KeyCode::Number4 => Key::D4,
                tcod::KeyCode::Number5 => Key::D5,
                tcod::KeyCode::Number6 => Key::D6,
                tcod::KeyCode::Number7 => Key::D7,
                tcod::KeyCode::Number8 => Key::D8,
                tcod::KeyCode::Number9 => Key::D9,
                // The numbers on the numeric keypad
                tcod::KeyCode::NumPad0 => Key::NumPad0,
                tcod::KeyCode::NumPad1 => Key::NumPad1,
                tcod::KeyCode::NumPad2 => Key::NumPad2,
                tcod::KeyCode::NumPad3 => Key::NumPad3,
                tcod::KeyCode::NumPad4 => Key::NumPad4,
                tcod::KeyCode::NumPad5 => Key::NumPad5,
                tcod::KeyCode::NumPad6 => Key::NumPad6,
                tcod::KeyCode::NumPad7 => Key::NumPad7,
                tcod::KeyCode::NumPad8 => Key::NumPad8,
                tcod::KeyCode::NumPad9 => Key::NumPad9,
                tcod::KeyCode::NumPadAdd => Key::NumPadPlus,
                tcod::KeyCode::NumPadSubtract => Key::NumPadMinus,
                tcod::KeyCode::NumPadDivide => Key::NumPadDivide,
                tcod::KeyCode::NumPadMultiply => Key::NumPadMultiply,
                tcod::KeyCode::NumPadDecimal => Key::NumPadDecimal,
                tcod::KeyCode::NumPadEnter => Key::NumPadEnter,
                tcod::KeyCode::F1 => Key::F1,
                tcod::KeyCode::F2 => Key::F2,
                tcod::KeyCode::F3 => Key::F3,
                tcod::KeyCode::F4 => Key::F4,
                tcod::KeyCode::F5 => Key::F5,
                tcod::KeyCode::F6 => Key::F6,
                tcod::KeyCode::F7 => Key::F7,
                tcod::KeyCode::F8 => Key::F8,
                tcod::KeyCode::F9 => Key::F9,
                tcod::KeyCode::F10 => Key::F10,
                tcod::KeyCode::F11 => Key::F11,
                tcod::KeyCode::F12 => Key::F12,
                tcod::KeyCode::NUMLOCK => Key::NumLockClear,
                tcod::KeyCode::SCROLLLOCK => Key::ScrollLock,
                tcod::KeyCode::Spacebar => Key::Space,
                // TODO: check if this is correct.
                tcod::KeyCode::Char => Key::Unknown,
            }
        }
    }
}

pub fn tcod_map_mouse(prev_state: tcod::MouseState, state: &tcod::MouseState) -> MouseButton {
    if prev_state.lbutton != state.lbutton { return MouseButton::Left }
    else if prev_state.rbutton != state.rbutton { return MouseButton::Right }
    else if prev_state.mbutton != state.mbutton { return MouseButton::Middle }
    else { return MouseButton::Unknown }
}
