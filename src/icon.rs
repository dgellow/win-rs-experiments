#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use crate::assert::{assert_ne, Result, WithLastWin32Error};
use windows::Win32::{Foundation::PWSTR, UI::WindowsAndMessaging::*};

pub fn load_icon(i: Icon) -> Result<HICON> {
	let icon = unsafe {
		match i {
			IconStr(a) => LoadIconW(0, a),
			IconId(_) => {
				return Err("do not know how to load u32 icons".into());
			}
		}
	};
	assert_ne(icon, 0, "failed to get icon handle").with_last_win32_err()?;

	Ok(icon)
}

pub enum Icon {
	IconStr(PWSTR),
	IconId(u32),
}

use Icon::*;

pub const Application: Icon = IconStr(IDI_APPLICATION);
pub const Asterisk: Icon = IconStr(IDI_ASTERISK);
pub const Error: Icon = IconId(IDI_ERROR);
pub const Exclamation: Icon = IconStr(IDI_EXCLAMATION);
pub const Hand: Icon = IconStr(IDI_HAND);
pub const Information: Icon = IconId(IDI_INFORMATION);
pub const Question: Icon = IconStr(IDI_QUESTION);
pub const Shield: Icon = IconStr(IDI_SHIELD);
pub const Warning: Icon = IconId(IDI_WARNING);
pub const WinLogo: Icon = IconStr(IDI_WINLOGO);
