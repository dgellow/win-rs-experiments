use windows::Win32::{
	Foundation::{HINSTANCE, HWND},
	System::LibraryLoader::LoadLibraryW,
	UI::WindowsAndMessaging::{
		CreateWindowExW, ES_MULTILINE, WINDOW_STYLE, WS_CHILD, WS_TABSTOP, WS_VISIBLE,
	},
};

use crate::{
	assert::{assert_ne, Result, WithLastWin32Error},
	wide_string::ToWide,
	Point,
};

const RICHEDIT_MODULE: &str = "msftedit.dll";
const RICHEDIT_CLASS: &str = "RICHEDIT50W";

pub fn create(owner: HWND, h_instance: HINSTANCE, position: Point, dimension: Point) -> Result<()> {
	assert_ne(
		unsafe { LoadLibraryW(RICHEDIT_MODULE.to_wide().as_pwstr()) },
		0,
		format!("failed to load library {}", RICHEDIT_MODULE).as_str(),
	)
	.with_last_win32_err()?;

	let multiline: WINDOW_STYLE = ES_MULTILINE
		.try_into()
		.expect("failed to cast ES_MULTILINE to u32");
	let styles = multiline | WS_VISIBLE | WS_CHILD | WS_TABSTOP;

	let control = unsafe {
		CreateWindowExW(
			0,
			RICHEDIT_CLASS.to_wide().as_pwstr(),
			"Type here".to_wide().as_pwstr(),
			styles,
			position.x,
			position.y,
			dimension.x,
			dimension.y,
			owner,
			None,
			h_instance,
			std::ptr::null(),
		)
	};
	assert_ne(control, 0, "failed to create rich_edit control").with_last_win32_err()?;

	Ok(())
}
