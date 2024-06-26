use windows::Win32::{Foundation::PWSTR, UI::WindowsAndMessaging::CW_USEDEFAULT};

pub mod assert;
pub mod button;
pub mod class;
pub mod color;
pub mod cursor;
pub mod icon;
pub mod input;
pub mod layout;
pub mod macros;
pub mod menu;
pub mod message_box;
pub mod rich_edit;
pub mod theme;
pub mod wide_string;
pub mod window;
pub mod window_long;

mod simple_app;
pub use crate::simple_app::*;

pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Default for Point {
	fn default() -> Self {
		Self {
			x: CW_USEDEFAULT,
			y: CW_USEDEFAULT,
		}
	}
}

pub fn null_pwstr() -> PWSTR {
	PWSTR(std::ptr::null_mut())
}

pub fn hiword(word: usize) -> usize {
	(word >> 16) & 0xffff
}

pub fn loword(word: usize) -> usize {
	word & 0xffff
}
