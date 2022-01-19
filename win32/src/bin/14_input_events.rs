use derive::WindowBase;
use gui::{
	assert::{assert_ne, Result, WithLastWin32Error},
	display, hiword,
	input::{self, style},
	loword,
	wide_string::ToWide,
	window::{self, message, MessageAction, Options, WindowBase, WindowHandler},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND},
	UI::WindowsAndMessaging::{CreateWindowExW, SetWindowLongW, GWL_ID, WINDOW_STYLE},
};

fn main() -> Result<()> {
	let app = App::new("Input Events — Win32 💖 Rust");
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

const EDIT_CLASS: &str = "EDIT";

impl WindowHandler for App {
	fn on_message(
		&mut self,
		message: window::message::Type,
		wparam: windows::Win32::Foundation::WPARAM,
		_lparam: windows::Win32::Foundation::LPARAM,
	) -> Result<MessageAction> {
		use MessageAction::*;
		match message {
			message::Create => return self.on_create(),
			message::Command => {
				let notif_code: input::event::Type = hiword(wparam).try_into().unwrap();
				let control_id: u32 = loword(wparam).try_into().unwrap();
				match notif_code {
					input::event::AfterPaste => display!("edit #{}: event AfterPaste", control_id),
					input::event::AlignLtrEc => display!("edit #{}: event AlignLtrEc", control_id),
					input::event::AlignRtlEc => display!("edit #{}: event AlignRtlEc", control_id),
					input::event::BeforePaste => {
						display!("edit #{}: event BeforePaste", control_id)
					}
					input::event::Change => display!("edit #{}: event Change", control_id),
					input::event::ErrSpace => display!("edit #{}: event ErrSpace", control_id),
					input::event::HScroll => display!("edit #{}: event HScroll", control_id),
					input::event::KillFocus => display!("edit #{}: event KillFocus", control_id),
					input::event::MaxText => display!("edit #{}: event MaxText", control_id),
					input::event::SetFocus => display!("edit #{}: event SetFocus", control_id),
					input::event::Update => display!("edit #{}: event Update", control_id),
					input::event::VScroll => display!("edit #{}: event VScroll", control_id),
					_ => display!("other command"),
				};
			}
			_ => {} // display!("other message => {:?}", other),
		};
		Ok(Continue)
	}

	fn on_create(&self) -> Result<MessageAction> {
		let styles = TryInto::<WINDOW_STYLE>::try_into(style::Left)
			.expect("cannot cast to WINDOW_STYLE")
			| (window::style::Visible | window::style::Child | window::style::Overlapped).0;
		let ex_styles = window::ex_style::ClientEdge
			| window::ex_style::Left
			| window::ex_style::LtrReading
			| window::ex_style::RightScrollbar;

		let control1 = unsafe {
			CreateWindowExW(
				ex_styles.0,
				EDIT_CLASS.to_wide().as_pwstr(),
				"Type text".to_wide().as_pwstr(),
				styles,
				10,
				10,
				100,
				20,
				self.h_window,
				None,
				self.h_instance,
				std::ptr::null(),
			)
		};
		assert_ne(control1, 0, "failed to create edit control").with_last_win32_err()?;
		unsafe { SetWindowLongW(control1, GWL_ID, 100) };

		let control2 = unsafe {
			CreateWindowExW(
				ex_styles.0,
				EDIT_CLASS.to_wide().as_pwstr(),
				"Type text".to_wide().as_pwstr(),
				styles,
				10,
				40,
				100,
				20,
				self.h_window,
				None,
				self.h_instance,
				std::ptr::null(),
			)
		};
		assert_ne(control2, 0, "failed to create edit control").with_last_win32_err()?;
		unsafe { SetWindowLongW(control2, GWL_ID, 101) };

		Ok(MessageAction::Continue)
	}
}
