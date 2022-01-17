use crate::{
	assert::Result,
	display,
	layout::{Control, Screen},
	window::{MessageAction, Options, WindowBase, WindowHandler},
};
use derive::WindowBase;
use windows::Win32::Foundation::{HINSTANCE, HWND};

#[derive(Debug, WindowBase)]
pub struct SimpleApp {
	h_instance: HINSTANCE,
	h_window: HWND,
	create_layout: fn() -> Control,
	title: String,
}

impl Default for SimpleApp {
	fn default() -> Self {
		Self {
			create_layout: || Control::None,
			title: Default::default(),
			h_instance: Default::default(),
			h_window: Default::default(),
		}
	}
}

impl SimpleApp {
	pub fn new(title: &str, create_layout: fn() -> Control) -> Self {
		Self {
			title: title.to_owned(),
			create_layout,
			..Default::default()
		}
	}

	pub fn run(&self) -> Result<()> {
		let main_window = Self::new_window(
			"MainWindow",
			self.title.as_str(),
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

impl WindowHandler for SimpleApp {
	fn on_create(&self) -> Result<MessageAction> {
		let root = (self.create_layout)();
		let mut screen = Screen::new(self.h_instance, self.h_window);
		screen.render(root)?;

		Ok(MessageAction::Continue)
	}
}
