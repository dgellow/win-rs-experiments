// Implement combobox subclassing example from https://docs.microsoft.com/en-us/windows/win32/controls/subclass-a-combo-box#complete-example

use std::{fmt::Debug, sync::Once};

use derive::WindowBase;
use gui::{
	assert::{assert_not_null, Result},
	display, err_display,
	window::{
		message, style, MessageAction, Options, WinProc, WindowBase, WindowCreateData,
		WindowHandler,
	},
	window_long::{get_property, set_property, set_window_long_ptr},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
	UI::{
		Input::KeyboardAndMouse::{GetFocus, SetFocus, VK_ESCAPE, VK_RETURN, VK_TAB},
		WindowsAndMessaging::{
			CallWindowProcW, GetWindow, SendMessageW, CBS_DROPDOWN, CB_ADDSTRING, CB_ERR,
			CB_FINDSTRINGEXACT, CB_GETCURSEL, CB_SETCURSEL, GWLP_WNDPROC, GW_CHILD, WINDOW_STYLE,
		},
	},
};

const APP_STATE_PROPERTY: &str = "my_app_state";

fn main() -> std::result::Result<(), ()> {
	let app = App::new("Control Subclass — Win32 💖 Rust");

	match app.run() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

#[derive(Default, WindowBase)]
pub struct App {
	h_instance: HINSTANCE,
	h_window: HWND,
	title: String,
	edit_base_win_proc: Option<WinProc>,
	h_combo1: Option<HWND>,
	h_combo2: Option<HWND>,
	h_edit1: Option<HWND>,
	h_edit2: Option<HWND>,
}

impl Debug for App {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("App")
			.field("h_instance", &self.h_instance)
			.field("h_window", &self.h_window)
			.field("title", &self.title)
			.field(
				"edit_base_win_proc",
				&match self.edit_base_win_proc {
					Some(_) => Some(()),
					None => None,
				},
			)
			.field("h_combo1", &self.h_combo1)
			.field("h_combo2", &self.h_combo2)
			.field("h_edit1", &self.h_edit1)
			.field("h_edit2", &self.h_edit2)
			.finish()
	}
}

impl WindowHandler for App {
	fn on_create_mut(&mut self) -> Result<MessageAction> {
		// 1. create two combobox
		let class_name = "COMBOBOX";
		let combo_style: WINDOW_STYLE = CBS_DROPDOWN.try_into().unwrap();
		let style = style::Child | style::Visible | style::Type(combo_style);

		let combo1 = self.create_window(
			class_name,
			None,
			None,
			Some(style),
			10,
			10,
			100,
			50,
			None,
			Some(self.h_window),
			WindowCreateData::None,
		)?;

		let combo2 = self.create_window(
			class_name,
			None,
			None,
			Some(style),
			120,
			10,
			100,
			50,
			Some(self.h_instance),
			Some(self.h_window),
			WindowCreateData::None,
		)?;

		// 2. get edit control handle from each combobox
		let edit1 = unsafe { GetWindow(combo1, GW_CHILD) };
		let edit2 = unsafe { GetWindow(combo2, GW_CHILD) };

		// 3. change the win-proc for both edit handlers. Both base win-proc are the same, we only need one pointer.
		//
		// Note: an alternative to SetWindowLongPtr + GWLP_WNDPROC is to use SetWindowSubclass.
		// See https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass
		let base_win_proc =
			set_window_long_ptr(edit1, GWLP_WNDPROC, Self::edit_win_proc as *mut isize as _)?;
		set_window_long_ptr(edit2, GWLP_WNDPROC, Self::edit_win_proc as *mut isize as _)?;

		// 4. pass state pointer to be available from win_proc
		//
		// Note: the base window may already be using GWLP_USERDATA, so it is safer to pass data another way
		//
		// Note: different ways to pass data to the win_proc are discussed here: https://stackoverflow.com/questions/117792/best-method-for-storing-this-pointer-for-use-in-wndproc
		set_property(edit1, APP_STATE_PROPERTY, self)?;
		set_property(edit2, APP_STATE_PROPERTY, self)?;

		// 5. keep references to combobox, edits, and base win-proc
		self.h_combo1 = Some(combo1);
		self.h_combo2 = Some(combo2);
		self.h_edit1 = Some(edit1);
		self.h_edit2 = Some(edit2);

		unsafe {
			self.edit_base_win_proc = Some(std::mem::transmute(base_win_proc));
		}

		Ok(MessageAction::Continue)
	}

	fn on_message(
		&mut self,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> Result<MessageAction> {
		use gui::window::MessageAction::*;

		match message {
			message::Create => self.on_create_mut(),

			message::Setfocus => {
				display!("on_message => SetFocus");
				unsafe { SetFocus(self.h_combo1.unwrap()) };
				Ok(FullyHandled)
			}

			app_message::Tab => {
				display!("on_message => Tab");
				unsafe {
					let focus = GetFocus();
					if focus == self.h_edit1.unwrap() {
						SetFocus(self.h_combo2.unwrap());
					} else if focus == self.h_edit2.unwrap() {
						SetFocus(self.h_combo1.unwrap());
					} else {
						return Ok(Continue);
					}
				}
				Ok(FullyHandled)
			}

			app_message::Esc => {
				display!("on_message => Esc");
				unsafe {
					let focus = GetFocus();
					let combo = if focus == self.h_edit1.unwrap() {
						self.h_combo1.unwrap()
					} else if focus == self.h_edit2.unwrap() {
						self.h_combo2.unwrap()
					} else {
						return Ok(Continue);
					};

					// clear selection and focus main window
					SendMessageW(
						combo,
						CB_SETCURSEL,
						WPARAM::MAX, /* equivalent to (WPARAM)(-1) */
						0,
					);

					SetFocus(self.h_window);
				}
				Ok(FullyHandled)
			}

			app_message::Enter => {
				display!("on_message => Enter");

				unsafe {
					let combo = if GetFocus() == self.h_edit1.unwrap() {
						self.h_combo1.unwrap()
					} else {
						self.h_combo2.unwrap()
					};
					SetFocus(self.h_window);

					//  if nothing is selected, select first item
					let cb_err: isize = CB_ERR.try_into().unwrap();
					let selected = SendMessageW(combo, CB_GETCURSEL, 0, 0);
					if selected == cb_err {
						let mut buffer: [u8; 256] = [0; 256];
						let text = SendMessageW(
							combo,
							message::GetText,
							buffer.len(),
							buffer.as_mut_ptr() as _,
						);
						if text == 0 {
							return Ok(FullyHandled);
						}

						let mut index = SendMessageW(
							combo,
							CB_FINDSTRINGEXACT,
							WPARAM::MAX,
							buffer.as_mut_ptr() as _,
						);

						// add string and select it
						if index == cb_err {
							index = SendMessageW(combo, CB_ADDSTRING, 0, buffer.as_mut_ptr() as _);
						}
						if index != cb_err {
							SendMessageW(combo, CB_SETCURSEL, index.try_into().unwrap(), 0);
						}
					}
				}

				Ok(FullyHandled)
			}

			_ => Ok(Continue),
		}
	}
}

impl App {
	pub fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
			..Default::default()
		}
	}

	pub fn run(&self) -> Result<()> {
		let _main_window = Self::new_window(
			"MainWindow",
			self.title.as_str(),
			Options {
				..Default::default()
			},
		)?;

		let res = Self::event_loop();
		display!("event_loop result: {} ({:#X})", res, res);

		Ok(())
	}

	/// # Safety
	///
	/// This function is full of thread unsafetiness and other dangerous stuff.
	unsafe extern "system" fn edit_win_proc(
		h_window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		static mut STATE: *const App = std::ptr::null();
		static INIT: Once = Once::new();
		INIT.call_once(|| {
			STATE = get_property(h_window, APP_STATE_PROPERTY).unwrap() as _;
		});

		assert_not_null(STATE, "edit_win_proc static STATE not initialized").unwrap();

		// pass Tab, Esc, and Return key events to main window
		match message {
			message::KeyDown => {
				display!("edit_win_proc => KeyDown");
				let key: u16 = wparam.try_into().unwrap();

				match key {
					VK_TAB => {
						display!("edit_win_proc => VK_TAB");
						SendMessageW((*STATE).h_window, app_message::Tab, wparam, lparam);
						return 0;
					}
					VK_ESCAPE => {
						display!("edit_win_proc => VK_ESCAPE");
						SendMessageW((*STATE).h_window, app_message::Esc, wparam, lparam);
						return 0;
					}
					VK_RETURN => {
						display!("edit_win_proc => VK_RETURN");
						SendMessageW((*STATE).h_window, app_message::Enter, wparam, lparam);
						return 0;
					}
					_ => {
						display!("edit_win_proc => KeyDown => key {}", key)
					}
				}
			}

			message::KeyUp | message::Char => {
				display!("edit_win_proc => KeyUp | Char");
				let char: u16 = wparam.try_into().unwrap();
				if let VK_TAB | VK_ESCAPE | VK_RETURN = char {
					return 0;
				}
			}

			_ => {}
		}

		// pass all other messages to base win-proc
		if (*STATE).edit_base_win_proc.is_none() {
			panic!("no state.edit_base_win_proc");
		}

		CallWindowProcW(
			(*STATE).edit_base_win_proc,
			h_window,
			message,
			wparam,
			lparam,
		)
	}
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod app_message {
	use windows::Win32::UI::WindowsAndMessaging::WM_USER;
	pub type Type = u32;

	pub const Tab: Type = WM_USER;
	pub const Esc: Type = WM_USER + 1;
	pub const Enter: Type = WM_USER + 2;
}
