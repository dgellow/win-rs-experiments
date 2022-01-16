use derive::WindowBase;
use gui::{
	assert::Result,
	display, err_display,
	window::{message, Options, WindowBase, WindowHandler},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
	Graphics::Gdi::ValidateRect,
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
		"Simplified Layout Window — Win32 💖 Rust",
		Options {
			..Default::default()
		},
	)?;
	display!("main_window: {:?}", main_window);

	let res = MainWindow::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

#[derive(Debug, WindowBase)]
struct MainWindow {
	h_instance: HINSTANCE,
}

impl WindowHandler for MainWindow {
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
				on_create(self.h_instance, h_window)?;
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

fn on_create(h_instance: HINSTANCE, h_window: HWND) -> Result<()> {
	use gui::layout::*;

	let root = VStack::new(
		10,
		vec![
			HStack::new(
				10,
				vec![
					InputText::new("hello").height(20).width(100).done(),
					InputText::new("world").height(20).width(100).done(),
				],
			)
			.done(),
			HStack::new(
				10,
				vec![
					InputText::new("hello").height(20).width(100).done(),
					InputText::new("world").height(20).width(100).done(),
				],
			)
			.done(),
			Button::new("My Button 1")
				.height(40)
				.width(100)
				.left_margin(30)
				.done(),
		],
	)
	.left_padding(10)
	.done();

	let mut screen = Screen::new(h_instance, h_window);
	screen.render(root)
}
