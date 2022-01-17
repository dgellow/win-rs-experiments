use derive::WindowBase;
use gui::{
	assert::Result,
	display, err_display,
	window::{MessageAction, Options, WindowBase, WindowHandler},
};
use windows::Win32::Foundation::{HINSTANCE, HWND};

fn main() -> std::result::Result<(), ()> {
	match App::run() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

#[derive(Debug, Default, WindowBase)]
struct App {
	h_instance: HINSTANCE,
	h_window: HWND,
}

impl App {
	fn run() -> Result<()> {
		let main_window = Self::new_window(
			"MainWindow",
			"Derived Layout Window — Win32 💖 Rust",
			Options {
				..Default::default()
			},
		)?;
		display!("main_window: {:?}", main_window);

		let res = Self::event_loop();
		display!("event_loop result: {} ({:#X})", res, res);

		Ok(())
	}
}

impl WindowHandler for App {
	fn on_create(&self) -> Result<MessageAction> {
		use gui::layout::*;

		let root = VStack::new()
			.left_padding(10)
			.spacing(10)
			.items(vec![
				HStack::new()
					.spacing(10)
					.items(vec![
						InputText::new("hello").height(20).width(100).done(),
						InputText::new("world").height(20).width(100).done(),
					])
					.done(),
				HStack::new()
					.spacing(10)
					.items(vec![
						InputText::new("hello").height(20).width(100).done(),
						InputText::new("world").height(20).width(100).done(),
					])
					.done(),
				Button::new("My Button 1")
					.height(40)
					.width(100)
					.left_margin(30)
					.done(),
			])
			.done();

		let mut screen = Screen::new(self.h_instance, self.h_window);
		screen.render(root)?;

		Ok(MessageAction::Continue)
	}
}
