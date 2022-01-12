use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub mod assert;
pub mod class;
pub mod color;
pub mod cursor;
pub mod debug;
pub mod icon;
pub mod menu;
pub mod message_box;
pub mod rich_edit;
pub mod wide_string;
pub mod window;

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
