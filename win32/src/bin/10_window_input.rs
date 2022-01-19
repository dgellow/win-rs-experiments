use gui::{
	assert::Result,
	button, display, err_display, input,
	window::{message, Options, WindowBase, WindowHandler},
};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};

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
	let main_window = MainWindow::new_window(
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
#[derive(Debug, Default)]
struct MainWindow {
	h_instance: HINSTANCE,
	h_window: HWND,
}

// 2. implement traits WindowBase and WindowHandler
impl WindowBase for MainWindow {
	fn init_state(h_instance: HINSTANCE) -> Self {
		Self {
			h_instance,
			..Default::default()
		}
	}

	fn h_instance(&self) -> HINSTANCE {
		self.h_instance
	}

	fn set_h_window(&mut self, h_window: HWND) {
		self.h_window = h_window;
	}

	fn h_window(&self) -> HWND {
		self.h_window
	}
}

impl WindowHandler for MainWindow {
	fn on_message(
		&mut self,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> Result<gui::window::MessageAction> {
		use gui::window::MessageAction::*;

		match message {
			message::Create => {
				display!("WM_CREATE");
				input::create_text_input(
					self.h_window,
					self.h_instance,
					"Type text",
					0,
					0,
					200,
					30,
				)?;
				input::create_text_input(
					self.h_window,
					self.h_instance,
					"Type text",
					0,
					40,
					200,
					30,
				)?;
				button::create(self.h_window, self.h_instance, "Click me!", 8, 80, 80, 60)?;
				Ok(Continue)
			}
			message::Paint => {
				display!("WM_PAINT");
				// unsafe { ValidateRect(self.h_window, std::ptr::null()) };
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
