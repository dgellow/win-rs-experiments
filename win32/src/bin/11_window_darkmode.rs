// See:
// - https://github.com/ysc3839/win32-darkmode/blob/master/win32-darkmode/win32-darkmode.cpp
// - https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-setwindowtheme
// - https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Controls/fn.SetWindowTheme.html

use derive::WindowBase;
use gui::{
	assert::{assert_eq, Result},
	display, hiword,
	input::{self},
	layout::*,
	loword,
	theme::is_dark_theme,
	wide_string::ToWide,
	window::{self, message, MessageAction, Options, WindowBase, WindowHandler},
};
use windows::Win32::{
	Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS, HINSTANCE, HWND, PWSTR},
	System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_DWORD, RRF_RT_REG_DWORD},
};

fn main() -> Result<()> {
	let app = App::new("Window Dark Mode — Win32 💖 Rust");
	app.run()
}

#[derive(Debug, Default, WindowBase)]
pub struct App {
	h_instance: HINSTANCE,
	h_window: HWND,
	title: String,
}

impl App {
	pub fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
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

impl WindowHandler for App {
	fn on_message(
		&self,
		message: window::message::Type,
		_wparam: windows::Win32::Foundation::WPARAM,
		lparam: windows::Win32::Foundation::LPARAM,
	) -> Result<MessageAction> {
		use MessageAction::*;
		match message {
			message::Create => return self.on_create(),

			// based on https://stackoverflow.com/questions/51334674/how-to-detect-windows-10-light-dark-mode-in-win32-application
			message::Wininichange => {
				if lparam == "ImmersiveColorSet".to_wide().as_pwstr() {
					display!("WM_WININICHANGE");
					if is_dark_theme()? {
						display!("Dark theme enabled!");
					} else {
						display!("Light theme enabled!");
					}
				}
			}

			_ => {} // display!("other message => {:?}", other),
		};
		Ok(Continue)
	}

	fn on_create(&self) -> Result<MessageAction> {
		if is_dark_theme()? {
			display!("Dark theme enabled!");
		} else {
			display!("Light theme enabled!");
		}

		let control = VStack::new()
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
		screen.render(control)?;

		Ok(MessageAction::Continue)
	}
}
