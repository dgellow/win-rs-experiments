use gui::{
	assert::Result,
	button, display, err_display, input,
	window::{message, Options, WindowBase},
	Point,
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
	Graphics::Gdi::ValidateRect,
	UI::Controls::SetWindowTheme,
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
		"Input Window — Win32 💖 Rust",
		Options {
			..Default::default()
		},
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
	) -> Result<gui::window::MessageAction> {
		use gui::window::MessageAction::*;

		match message {
			message::Create => {
				display!("WM_CREATE");
				input::create_text_input(
					h_window,
					self.h_instance,
					Point { x: 0, y: 0 },
					Point { x: 200, y: 30 },
				)?;
				input::create_text_input(
					h_window,
					self.h_instance,
					Point { x: 0, y: 40 },
					Point { x: 200, y: 30 },
				)?;
				button::create(
					h_window,
					self.h_instance,
					"Click me!",
					Point { x: 8, y: 80 },
					Point { x: 80, y: 60 },
				)?;
				Ok(Continue)
			}
			message::Paint => {
				display!("WM_PAINT");
				unsafe { ValidateRect(h_window, std::ptr::null()) };
				Ok(Continue)
			}
			message::MButtonDown => {
				display!("WM_MBUTTONDOWN");
				Ok(Continue)
			}
			_ => Ok(Continue),
		}
	}
}
