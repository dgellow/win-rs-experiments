use windows::Win32::{
	Foundation::{HINSTANCE, HWND},
	UI::WindowsAndMessaging::*,
};

use crate::{
	assert::{assert_ne, Result, WithLastWin32Error},
	impl_ops_for_all,
	wide_string::ToWide,
	window,
};

const BUTTON_CLASS: &str = "BUTTON";

pub fn create(
	owner: HWND,
	h_instance: HINSTANCE,
	title: &str,
	x: i32,
	y: i32,
	width: i32,
	height: i32,
) -> Result<()> {
	let btn_styles: WINDOW_STYLE = (style::PushButton | style::Text)
		.0
		.try_into()
		.expect("cannot cast to WINDOW_STYLE");
	let styles = window::style::Visible | window::style::Child | window::style::Overlapped;
	let ex_styles =
		window::ex_style::Left | window::ex_style::LtrReading | window::ex_style::RightScrollbar;

	let control = unsafe {
		CreateWindowExW(
			ex_styles.0,
			BUTTON_CLASS.to_wide().as_pwstr(),
			title.to_wide().as_pwstr(),
			btn_styles | styles.0,
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
	assert_ne(control, 0, "failed to create button control").with_last_win32_err()?;

	Ok(())
}

impl_ops_for_all!(style::Type, message::Type);

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub struct Type(pub i32);

	pub const BS_3State: Type = Type(BS_3STATE);
	pub const Auto3state: Type = Type(BS_AUTO3STATE);
	pub const AutoCheckbox: Type = Type(BS_AUTOCHECKBOX);
	pub const AutoRadioButton: Type = Type(BS_AUTORADIOBUTTON);
	pub const Bitmap: Type = Type(BS_BITMAP);
	pub const Bottom: Type = Type(BS_BOTTOM);
	pub const Center: Type = Type(BS_CENTER);
	pub const Checkbox: Type = Type(BS_CHECKBOX);
	pub const DefPushButton: Type = Type(BS_DEFPUSHBUTTON);
	pub const Flat: Type = Type(BS_FLAT);
	pub const GroupBox: Type = Type(BS_GROUPBOX);
	pub const Icon: Type = Type(BS_ICON);
	pub const Left: Type = Type(BS_LEFT);
	pub const LeftText: Type = Type(BS_LEFTTEXT);
	pub const Multiline: Type = Type(BS_MULTILINE);
	pub const Notify: Type = Type(BS_NOTIFY);
	pub const OwnerDraw: Type = Type(BS_OWNERDRAW);
	pub const Pushbox: Type = Type(BS_PUSHBOX);
	pub const PushButton: Type = Type(BS_PUSHBUTTON);
	pub const PushLike: Type = Type(BS_PUSHLIKE);
	pub const RadioButton: Type = Type(BS_RADIOBUTTON);
	pub const Right: Type = Type(BS_RIGHT);
	pub const RightButton: Type = Type(BS_RIGHTBUTTON);
	pub const Text: Type = Type(BS_TEXT);
	pub const Top: Type = Type(BS_TOP);
	pub const Typemask: Type = Type(BS_TYPEMASK);
	pub const UserButton: Type = Type(BS_USERBUTTON);
	pub const VCenter: Type = Type(BS_VCENTER);
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod message {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub struct Type(pub u32);

	pub const Click: Type = Type(BM_CLICK);
	pub const GetCheck: Type = Type(BM_GETCHECK);
	pub const GetImage: Type = Type(BM_GETIMAGE);
	pub const GetState: Type = Type(BM_GETSTATE);
	pub const SetCheck: Type = Type(BM_SETCHECK);
	pub const SetDontClick: Type = Type(BM_SETDONTCLICK);
	pub const SetImage: Type = Type(BM_SETIMAGE);
	pub const SetState: Type = Type(BM_SETSTATE);
	pub const SetStyle: Type = Type(BM_SETSTYLE);
}

pub const _BN_CLICKED: u32 = BN_CLICKED;
pub const _BN_DBLCLK: u32 = BN_DBLCLK;
pub const _BN_DISABLE: u32 = BN_DISABLE;
pub const _BN_DOUBLECLICKED: u32 = BN_DOUBLECLICKED;
pub const _BN_HILITE: u32 = BN_HILITE;
pub const _BN_KILLFOCUS: u32 = BN_KILLFOCUS;
pub const _BN_PAINT: u32 = BN_PAINT;
pub const _BN_PUSHED: u32 = BN_PUSHED;
pub const _BN_SETFOCUS: u32 = BN_SETFOCUS;
pub const _BN_UNHILITE: u32 = BN_UNHILITE;
pub const _BN_UNPUSHED: u32 = BN_UNPUSHED;
pub const _BROADCAST_QUERY_DENY: u32 = BROADCAST_QUERY_DENY;
pub const _BSM_INSTALLABLEDRIVERS: u32 = BSM_INSTALLABLEDRIVERS;
pub const _BSM_NETDRIVER: u32 = BSM_NETDRIVER;
pub const _BSM_VXDS: u32 = BSM_VXDS;
pub const _BST_FOCUS: u32 = BST_FOCUS;
pub const _BST_PUSHED: u32 = BST_PUSHED;
