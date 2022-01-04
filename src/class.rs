#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = WNDCLASS_STYLES;

	pub const VRedraw: Type = CS_VREDRAW;
	pub const HRedraw: Type = CS_HREDRAW;
	pub const Dblclks: Type = CS_DBLCLKS;
	pub const OwndC: Type = CS_OWNDC;
	pub const ClassDc: Type = CS_CLASSDC;
	pub const ParentDc: Type = CS_PARENTDC;
	pub const NoClose: Type = CS_NOCLOSE;
	pub const SaveBits: Type = CS_SAVEBITS;
	pub const ByteAlignClient: Type = CS_BYTEALIGNCLIENT;
	pub const ByteAlignWindow: Type = CS_BYTEALIGNWINDOW;
	pub const GlobalClass: Type = CS_GLOBALCLASS;
	pub const Ime: Type = CS_IME;
	pub const DropShadow: Type = CS_DROPSHADOW;
}
