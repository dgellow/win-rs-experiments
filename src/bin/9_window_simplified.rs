use gui::{
	assert::Result,
	cursor, display, err_display, icon,
	window::{self, message, WindowBase},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
	Graphics::Gdi::ValidateRect,
	UI::WindowsAndMessaging::COLOR_BACKGROUND,
};

fn main() -> std::result::Result<(), ()> {
	match app() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

fn app() -> Result<()> {
	let main_window = MainWindow::new(
		"MainWindow",
		"Simplified Window — Win32 💖 Rust",
		Some(window::Options {
			icon: icon::WinLogo,
			cursor: cursor::Person,
			bg_brush: COLOR_BACKGROUND,
			..Default::default()
		}),
	)?;
	display!("main_window: {:?}", main_window);

	let res = MainWindow::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

// 1. create our window type
#[derive(Debug)]
struct MainWindow {
	h_instance: HINSTANCE,
}

// 2. implement WindowBase trait
impl WindowBase for MainWindow {
	fn init_state(h_instance: HINSTANCE) -> Self {
		Self { h_instance }
	}

	fn h_instance(&self) -> HINSTANCE {
		self.h_instance
	}

	fn on_message(
		&self,
		h_window: HWND,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> gui::window::MessageAction {
		use gui::window::MessageAction::*;

		match message {
			message::Paint => {
				display!("WM_PAINT");
				unsafe { ValidateRect(h_window, std::ptr::null()) };
				Continue
			}
			message::MButtonDown => {
				display!("WM_MBUTTONDOWN");
				Continue
			}
			_ => Continue,
		}
	}
}
