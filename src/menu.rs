#![allow(dead_code)]
#![allow(non_upper_case_globals)]
pub mod item_flag {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = MENU_ITEM_FLAGS;

	pub const ByCommand: Type = MF_BYCOMMAND;
	pub const ByPosition: Type = MF_BYPOSITION;
	pub const Bitmap: Type = MF_BITMAP;
	pub const Checked: Type = MF_CHECKED;
	pub const Disabled: Type = MF_DISABLED;
	pub const Enabled: Type = MF_ENABLED;
	pub const Grayed: Type = MF_GRAYED;
	pub const MenuBarBreak: Type = MF_MENUBARBREAK;
	pub const MenuBreak: Type = MF_MENUBREAK;
	pub const OwnerDraw: Type = MF_OWNERDRAW;
	pub const Popup: Type = MF_POPUP;
	pub const Separator: Type = MF_SEPARATOR;
	pub const String: Type = MF_STRING;
	pub const Unchecked: Type = MF_UNCHECKED;
	pub const Insert: Type = MF_INSERT;
	pub const Change: Type = MF_CHANGE;
	pub const Append: Type = MF_APPEND;
	pub const Delete: Type = MF_DELETE;
	pub const Remove: Type = MF_REMOVE;
	pub const UseCheckBitmaps: Type = MF_USECHECKBITMAPS;
	pub const UnHilite: Type = MF_UNHILITE;
	pub const Hilite: Type = MF_HILITE;
	pub const Default: Type = MF_DEFAULT;
	pub const Sysmenu: Type = MF_SYSMENU;
	pub const Help: Type = MF_HELP;
	pub const RightJustify: Type = MF_RIGHTJUSTIFY;
	pub const MouseSelect: Type = MF_MOUSESELECT;
	pub const End: Type = MF_END;
}
