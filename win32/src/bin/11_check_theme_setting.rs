// See:
// - https://github.com/ysc3839/win32-darkmode/blob/master/win32-darkmode/win32-darkmode.cpp
// - https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-setwindowtheme
// - https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Controls/fn.SetWindowTheme.html

use derive::WindowBase;
use gui::{
	assert::Result,
	display,
	theme::{app_theme_settings, Theme},
	wide_string::ToWide,
	window::{self, message, MessageAction, Options, WindowBase, WindowHandler},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, PWSTR},
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
			message::Settingchange => unsafe {
				let pwstr = lparam as *mut PWSTR;
				if *pwstr == "ImmersiveColorSet".to_wide().as_pwstr() {
					display!("WM_SETTINGSCHANGE");
					match app_theme_settings()? {
						Theme::Light => display!("Light theme enabled!"),
						Theme::Dark => display!("Dark theme enabled!"),
						Theme::Unknown => display!("Cannot identify current theme"),
					};
				}
			},

			_ => {} // display!("other message => {:?}", other),
		};
		Ok(Continue)
	}

	fn on_create(&self) -> Result<MessageAction> {
		match app_theme_settings()? {
			Theme::Light => display!("Light theme enabled!"),
			Theme::Dark => display!("Dark theme enabled!"),
			Theme::Unknown => display!("Cannot identify current theme"),
		};

		Ok(MessageAction::Continue)
	}
}
