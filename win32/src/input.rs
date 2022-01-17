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
	let styles = TryInto::<WINDOW_STYLE>::try_into(style::Left)
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
	pub type Type = u32;

	pub const LeftMargin: Type = EC_LEFTMARGIN;
	pub const RightMargin: Type = EC_RIGHTMARGIN;
	pub const UseFontInfo: Type = EC_USEFONTINFO;
}

// ??
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod edd {
	use windows::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;
	pub type Type = u32;

	pub const GetDeviceInterfaceName: Type = EDD_GET_DEVICE_INTERFACE_NAME;
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod feature {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = EDIT_CONTROL_FEATURE;

	pub const EnterpriseDataProtectionPasteSupport: Type =
		EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT;
	pub const PasteNotifications: Type = EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS;
}

// ???
#[allow(dead_code)]
#[allow(non_upper_case_globals)]

pub mod eds {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = u32;

	pub const RawMode: Type = EDS_RAWMODE;
	pub const RotatedMode: Type = EDS_ROTATEDMODE;
}

// ???
#[allow(dead_code)]
#[allow(non_upper_case_globals)]

pub mod eimes {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = u32;

	pub const CancelCompStrInFocus: Type = EIMES_CANCELCOMPSTRINFOCUS;
	pub const CompleteCompStrKillFocus: Type = EIMES_COMPLETECOMPSTRKILLFOCUS;
	pub const GetCompStrAtOnce: Type = EIMES_GETCOMPSTRATONCE;
	pub const CompositionString: Type = EMSIS_COMPOSITIONSTRING;
}
// event messages sent to win_proc
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
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
#[allow(dead_code)]
#[allow(non_upper_case_globals)]

pub mod end_session {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = u32;

	pub const CloseApp: Type = ENDSESSION_CLOSEAPP;
	pub const Critical: Type = ENDSESSION_CRITICAL;
	pub const LogOff: Type = ENDSESSION_LOGOFF;
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = i32;

	pub const AutoHScroll: Type = ES_AUTOHSCROLL;
	pub const AutoVScroll: Type = ES_AUTOVSCROLL;
	pub const Center: Type = ES_CENTER;
	pub const Left: Type = ES_LEFT;
	pub const Lowercase: Type = ES_LOWERCASE;
	pub const Multiline: Type = ES_MULTILINE;
	pub const NoHideSel: Type = ES_NOHIDESEL;
	pub const Number: Type = ES_NUMBER;
	pub const OemConvert: Type = ES_OEMCONVERT;
	pub const Password: Type = ES_PASSWORD;
	pub const Readonly: Type = ES_READONLY;
	pub const Right: Type = ES_RIGHT;
	pub const Uppercase: Type = ES_UPPERCASE;
	pub const WantReturn: Type = ES_WANTRETURN;
}
