use windows::Win32::{
	Foundation::{HINSTANCE, HWND},
	UI::WindowsAndMessaging::{CreateWindowExW, WINDOW_STYLE},
};

use crate::{
	assert::{assert_ne, Result, WithLastWin32Error},
	wide_string::ToWide,
	window,
};

const EDIT_CLASS: &str = "EDIT";

pub fn create_text_input(
	owner: HWND,
	h_instance: HINSTANCE,
	text: &str,
	x: i32,
	y: i32,
	width: i32,
	height: i32,
) -> Result<()> {
	let styles = TryInto::<WINDOW_STYLE>::try_into(style::ES_LEFT)
		.expect("cannot cast to WINDOW_STYLE")
		| (window::style::Visible | window::style::Child | window::style::Overlapped).0;
	let ex_styles = window::ex_style::ClientEdge
		| window::ex_style::Left
		| window::ex_style::LtrReading
		| window::ex_style::RightScrollbar;
	let control = unsafe {
		CreateWindowExW(
			ex_styles.0,
			EDIT_CLASS.to_wide().as_pwstr(),
			text.to_wide().as_pwstr(),
			styles,
			x,
			y,
			width,
			height,
			owner,
			None,
			h_instance,
			std::ptr::null(),
		)
	};
	assert_ne(control, 0, "failed to create edit control").with_last_win32_err()?;

	Ok(())
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod option {
	use windows::Win32::UI::WindowsAndMessaging::*;

	pub const LeftMargin: u32 = EC_LEFTMARGIN;
	pub const RightMargin: u32 = EC_RIGHTMARGIN;
	pub const UseFontInfo: u32 = EC_USEFONTINFO;
}

// ??
pub mod edd {
	pub const EDD_GET_DEVICE_INTERFACE_NAME: u32 = 1u32;
}

pub mod feature {
	use windows::Win32::UI::WindowsAndMessaging::*;

	pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE =
		0i32;
	pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE = 1i32;
}

// ???
pub mod eds {
	pub const EDS_RAWMODE: u32 = 2u32;
	pub const EDS_ROTATEDMODE: u32 = 4u32;
}

// ???
pub mod eimes {
	pub const EIMES_CANCELCOMPSTRINFOCUS: u32 = 2u32;
	pub const EIMES_COMPLETECOMPSTRKILLFOCUS: u32 = 4u32;
	pub const EIMES_GETCOMPSTRATONCE: u32 = 1u32;
	pub const EMSIS_COMPOSITIONSTRING: u32 = 1u32;
}
// event messages sent to win_proc
pub mod event {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = u32;

	pub const AfterPaste: Type = EN_AFTER_PASTE;
	pub const AlignLtrEc: Type = EN_ALIGN_LTR_EC;
	pub const AlignRtlEc: Type = EN_ALIGN_RTL_EC;
	pub const BeforePaste: Type = EN_BEFORE_PASTE;
	pub const Change: Type = EN_CHANGE;
	pub const ErrSpace: Type = EN_ERRSPACE;
	pub const HScroll: Type = EN_HSCROLL;
	pub const KillFocus: Type = EN_KILLFOCUS;
	pub const MaxText: Type = EN_MAXTEXT;
	pub const SetFocus: Type = EN_SETFOCUS;
	pub const Update: Type = EN_UPDATE;
	pub const VScroll: Type = EN_VSCROLL;
}

// ???
pub mod end_session {
	pub const ENDSESSION_CLOSEAPP: u32 = 1u32;
	pub const ENDSESSION_CRITICAL: u32 = 1073741824u32;
	pub const ENDSESSION_LOGOFF: u32 = 2147483648u32;
}

pub mod style {
	pub const ES_AUTOHSCROLL: i32 = 128i32;
	pub const ES_AUTOVSCROLL: i32 = 64i32;
	pub const ES_CENTER: i32 = 1i32;
	pub const ES_LEFT: i32 = 0i32;
	pub const ES_LOWERCASE: i32 = 16i32;
	pub const ES_MULTILINE: i32 = 4i32;
	pub const ES_NOHIDESEL: i32 = 256i32;
	pub const ES_NUMBER: i32 = 8192i32;
	pub const ES_OEMCONVERT: i32 = 1024i32;
	pub const ES_PASSWORD: i32 = 32i32;
	pub const ES_READONLY: i32 = 2048i32;
	pub const ES_RIGHT: i32 = 2i32;
	pub const ES_UPPERCASE: i32 = 8i32;
	pub const ES_WANTRETURN: i32 = 4096i32;
}
