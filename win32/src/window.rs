use std::sync::Once;

use crate::{
	assert::{assert_eq, assert_ne, assert_not_null, Result, WithLastWin32Error, WrappedError},
	cursor::{self, load_cursor},
	display,
	icon::{self, load_icon, Icon},
	impl_ops_for_all,
	theme::{app_theme_settings, Theme},
	wide_string::ToWide,
	window_long::{get_window_long_ptr, set_window_long_ptr},
};
use windows::Win32::{
	Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
	Graphics::Gdi::{GetStockObject, UpdateWindow, DEFAULT_GUI_FONT},
	System::LibraryLoader::GetModuleHandleExW,
	UI::WindowsAndMessaging::*,
};

pub type WinProc =
	unsafe extern "system" fn(window: HWND, message: message::Type, WPARAM, LPARAM) -> LRESULT;

impl_ops_for_all!(class_style::Type, style::Type, ex_style::Type);

pub enum MessageAction {
	None,
	Continue,
	FullyHandled,
}

pub struct Options {
	pub bg_brush: u32,
	pub class_style: class_style::Type,
	pub cursor: cursor::Type,
	pub height: i32,
	pub icon: Icon,
	pub width: i32,
	pub window_ext_style: ex_style::Type,
	pub window_style: style::Type,
	pub x: i32,
	pub y: i32,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			x: CW_USEDEFAULT,
			y: CW_USEDEFAULT,
			width: 500,
			height: 400,
			class_style: class_style::HRedraw | class_style::VRedraw,
			window_ext_style: ex_style::OverlappedWindow,
			window_style: style::OverlappedWindow,
			bg_brush: COLOR_WINDOW + 1,
			cursor: cursor::Arrow,
			icon: icon::Application,
		}
	}
}

pub trait WindowBase: Default {
	fn init_state(h_instance: HINSTANCE) -> Self;
	fn h_instance(&self) -> HINSTANCE;
	fn set_h_window(&mut self, h_window: HWND);
	fn h_window(&self) -> HWND;

	#[allow(clippy::too_many_arguments)]
	fn create_window(
		&mut self,
		class_name: &str,
		text: Option<&str>,
		ex_style: Option<ex_style::Type>,
		style: Option<style::Type>,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		h_instance: Option<HINSTANCE>,
		parent: Option<HWND>,
		set_create_data: bool,
	) -> Result<HWND> {
		let hwnd = unsafe {
			CreateWindowExW(
				if let Some(s) = ex_style { s.0 } else { 0 },
				class_name.to_wide().as_pwstr(),
				if let Some(t) = text { t } else { "" }.to_wide().as_pwstr(),
				if let Some(s) = style { s.0 } else { 0 },
				x,
				y,
				width,
				height,
				parent,
				None,
				h_instance,
				// pass ptr-to-self to win-proc, via WM_CREATE lparam
				if set_create_data {
					self as *mut _ as _
				} else {
					std::ptr::null_mut()
				},
			)
		};
		assert_ne(
			hwnd,
			0,
			format!("failed to create window for class {:?}", class_name).as_str(),
		)
		.with_last_win32_err()?;
		Ok(hwnd)
	}
}

pub trait WindowHandler: WindowBase
where
	Self: Sized,
{
	fn new_window<Opts>(class_name: &str, title: &str, options: Opts) -> Result<Box<Self>>
	where
		Self: Sized,
		Opts: Into<Option<Options>>,
	{
		let opts = options.into().unwrap_or_default();

		let mut h_instance: HINSTANCE = Default::default();
		assert_eq(
			unsafe { GetModuleHandleExW(0, None, &mut h_instance as *mut _) },
			BOOL(1),
			"failed to get module handle",
		)
		.with_last_win32_err()?;

		let size: u32 = std::mem::size_of::<WNDCLASSEXW>()
			.try_into()
			.expect("WNDCLASSEXW size not u32");

		let icon = load_icon(opts.icon)?;
		let wnd_class = WNDCLASSEXW {
			cbSize: size,
			style: opts.class_style.0,
			lpfnWndProc: Some(Self::win_proc),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: h_instance,
			hIcon: icon,
			hCursor: load_cursor(opts.cursor)?,
			hbrBackground: opts
				.bg_brush
				.try_into()
				.expect("cannot convert color to HBRUSH"),
			lpszMenuName: Default::default(), // defaults to null
			lpszClassName: class_name.to_wide().as_pwstr(),
			hIconSm: icon,
		};

		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		let mut state = Box::new(Self::init_state(h_instance));

		let h_window = state.create_window(
			class_name,
			Some(title),
			Some(opts.window_ext_style),
			Some(opts.window_style),
			opts.x,
			opts.y,
			opts.width,
			opts.height,
			Some(h_instance),
			None,
			true,
		)?;
		// CreateWindowExW(
		// 	opts.window_ext_style.0,
		// 	class_name.to_wide().as_pwstr(),
		// 	title.to_wide().as_pwstr(),
		// 	opts.window_style.0,
		// 	opts.x,
		// 	opts.y,
		// 	opts.width,
		// 	opts.height,
		// 	None,
		// 	None,
		// 	h_instance,
		// 	&mut state as *mut _ as _,
		// )
		// assert_ne(h_window, 0, "failed to create window").with_last_win32_err()?;

		unsafe { ShowWindow(h_window, show_cmd::Show) };
		unsafe { UpdateWindow(h_window) };

		display!("new_window => state addr {:p}", &state);

		Ok(state)
	}

	// TODO: find better name
	// TODO: should only be called once per app (or top window?)
	fn set_window_state_from_lparam(h_window: HWND, lparam: LPARAM) -> Result<*mut Self> {
		let create_struct = lparam as *mut CREATESTRUCTW;
		assert_not_null(create_struct, "WM_CREATE lparam cannot be null")?;
		unsafe {
			let state = (*create_struct).lpCreateParams as *mut Self;
			(*state).set_h_window(h_window);
			set_window_long_ptr(h_window, GWLP_USERDATA, state as _)
				.wrap_err("set_state_from_lparam failed")?;

			Ok(state)
		}
	}

	// TODO: find better name
	// TODO: should the child have its own state instead of a pointer to the whole app?
	fn set_child_state(&mut self, child: HWND) -> Result<()> {
		let state: isize = self as *mut _ as _;
		set_window_long_ptr(child, GWLP_USERDATA, state).wrap_err("set_child_state failed")?;
		Ok(())
	}

	// TODO: find better name
	fn get_window_state(h_window: HWND) -> Result<*mut Self> {
		let state =
			get_window_long_ptr(h_window, GWLP_USERDATA).wrap_err("get_state failed")? as *mut Self;
		assert_not_null(state, "GWLP_USERDATA cannot be null")?;
		Ok(state)
	}

	fn event_loop() -> WPARAM {
		let mut msg: MSG = Default::default();
		let msg_ptr: *mut MSG = &mut msg as *mut _;
		unsafe {
			while GetMessageW(msg_ptr, 0, 0, 0).as_bool() {
				TranslateMessage(msg_ptr);
				DispatchMessageW(msg_ptr);
			}
			(*msg_ptr).wParam
		}
	}

	/// # Safety
	///
	/// This function is full of thread unsafetiness and other dangerous stuff.
	unsafe extern "system" fn win_proc(
		h_window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		use MessageAction::*;
		static mut USER_DATA: isize = 0;

		let default_win_proc = || DefWindowProcW(h_window, message, wparam, lparam);

		match message {
			message::Create => {
				let state = Self::set_window_state_from_lparam(h_window, lparam).unwrap();
				USER_DATA = state as _;
			}
			// called before Create, no state has yet been associated with window handler
			message::GetMinmaxinfo | message::NcCreate | message::NcCalcSize => {
				return default_win_proc();
			}
			// some very noisy messages that can be generally ignored
			message::NcHitTest | message::NcMouseMove | message::Setcursor => {
				return default_win_proc();
			}
			message::Destroy => {
				PostQuitMessage(0);
				// 0
				// std::ptr::null_mut()
			}

			_ => {}
		};

		assert_ne(USER_DATA, 0, "no win_proc static USER_DATA").unwrap();
		let state: *mut Self = USER_DATA as _;

		// display!("win_proc => message {:?}", message);
		// display!("win_proc => USER_DATA {:?}", USER_DATA);
		// display!("win_proc => state.h_instance {:?}", (*state).h_instance());

		let action = (*state).on_message(message, wparam, lparam).unwrap();

		// set font to all controls
		if message == message::Create {
			let font = GetStockObject(DEFAULT_GUI_FONT);
			EnumChildWindows(h_window, Some(Self::set_font), font);
		}

		display!("");

		match action {
			Continue | None => default_win_proc(),
			FullyHandled => 0,
		}
	}

	extern "system" fn set_font(child: HWND, font: LPARAM) -> BOOL {
		unsafe {
			SendMessageW(
				child,
				message::Setfont,
				font.try_into().expect("cannot cast LPARAM font to usize"),
				1, // => true
			);
		}
		BOOL(1)
	}

	fn on_message(
		&mut self,
		message: message::Type,
		_wparam: WPARAM,
		lparam: LPARAM,
	) -> Result<MessageAction> {
		use MessageAction::*;

		// display!("enter on_message");

		match message {
			message::Create => {
				display!("WM_CREATE");
				match self.on_create_mut()? {
					None => self.on_create(),
					other => Ok(other),
				}
			}
			message::Paint => {
				display!("WM_PAINT");
				self.on_paint()
			}
			message::Size => {
				display!("WM_SIZE");
				self.on_size()
			}
			message::Settingchange => unsafe {
				let pwstr = lparam as *mut PWSTR;
				if *pwstr == "ImmersiveColorSet".to_wide().as_pwstr() {
					self.on_theme_change(app_theme_settings()?)
				} else {
					Ok(Continue)
				}
			},
			_ => Ok(Continue),
		}
	}

	fn on_create_mut(&mut self) -> Result<MessageAction> {
		Ok(MessageAction::None)
	}

	fn on_create(&self) -> Result<MessageAction> {
		Ok(MessageAction::None)
	}

	fn on_paint(&self) -> Result<MessageAction> {
		// unsafe { ValidateRect(self.h_window(), std::ptr::null()) };
		Ok(MessageAction::None)
	}

	fn on_size(&self) -> Result<MessageAction> {
		Ok(MessageAction::None)
	}

	fn on_theme_change(&self, _app_theme: Theme) -> Result<MessageAction> {
		Ok(MessageAction::None)
	}
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod class_style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub struct Type(pub WNDCLASS_STYLES);

	pub const VRedraw: Type = Type(CS_VREDRAW);
	pub const HRedraw: Type = Type(CS_HREDRAW);
	pub const Dblclks: Type = Type(CS_DBLCLKS);
	pub const OwndC: Type = Type(CS_OWNDC);
	pub const ClassDc: Type = Type(CS_CLASSDC);
	pub const ParentDc: Type = Type(CS_PARENTDC);
	pub const NoClose: Type = Type(CS_NOCLOSE);
	pub const SaveBits: Type = Type(CS_SAVEBITS);
	pub const ByteAlignClient: Type = Type(CS_BYTEALIGNCLIENT);
	pub const ByteAlignWindow: Type = Type(CS_BYTEALIGNWINDOW);
	pub const GlobalClass: Type = Type(CS_GLOBALCLASS);
	pub const Ime: Type = Type(CS_IME);
	pub const DropShadow: Type = Type(CS_DROPSHADOW);
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod message {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = u32;

	pub const Activate: Type = WM_ACTIVATE;
	pub const ActivateApp: Type = WM_ACTIVATEAPP;
	pub const AfxFirst: Type = WM_AFXFIRST;
	pub const AfxLast: Type = WM_AFXLAST;
	pub const App: Type = WM_APP;
	pub const AppCommand: Type = WM_APPCOMMAND;
	pub const AskCbFormatName: Type = WM_ASKCBFORMATNAME;
	pub const CancelJournal: Type = WM_CANCELJOURNAL;
	pub const CancelMode: Type = WM_CANCELMODE;
	pub const CaptureChanged: Type = WM_CAPTURECHANGED;
	pub const ChangeCbChain: Type = WM_CHANGECBCHAIN;
	pub const ChangeUiState: Type = WM_CHANGEUISTATE;
	pub const Char: Type = WM_CHAR;
	pub const CharToItem: Type = WM_CHARTOITEM;
	pub const ChildActivate: Type = WM_CHILDACTIVATE;
	pub const Clear: Type = WM_CLEAR;
	pub const ClipboardUpdate: Type = WM_CLIPBOARDUPDATE;
	pub const Close: Type = WM_CLOSE;
	pub const Command: Type = WM_COMMAND;
	pub const CommNotify: Type = WM_COMMNOTIFY;
	pub const Compacting: Type = WM_COMPACTING;
	pub const CompareItem: Type = WM_COMPAREITEM;
	pub const Copy: Type = WM_COPY;
	pub const Copydata: Type = WM_COPYDATA;
	pub const Create: Type = WM_CREATE;
	pub const CtlColorBtn: Type = WM_CTLCOLORBTN;
	pub const CtlColorDlg: Type = WM_CTLCOLORDLG;
	pub const CtlColorEdit: Type = WM_CTLCOLOREDIT;
	pub const CtlColorListbox: Type = WM_CTLCOLORLISTBOX;
	pub const CtlColorMsgBox: Type = WM_CTLCOLORMSGBOX;
	pub const CtlColorScrollbar: Type = WM_CTLCOLORSCROLLBAR;
	pub const CtlColorStatic: Type = WM_CTLCOLORSTATIC;
	pub const Cut: Type = WM_CUT;
	pub const DeadChar: Type = WM_DEADCHAR;
	pub const DeleteItem: Type = WM_DELETEITEM;
	pub const Destroy: Type = WM_DESTROY;
	pub const DestroyClipboard: Type = WM_DESTROYCLIPBOARD;
	pub const DeviceChange: Type = WM_DEVICECHANGE;
	pub const DevModeChange: Type = WM_DEVMODECHANGE;
	pub const DisplayChange: Type = WM_DISPLAYCHANGE;
	pub const DpiChanged: Type = WM_DPICHANGED;
	pub const DpiChangedAfterParent: Type = WM_DPICHANGED_AFTERPARENT;
	pub const DpiChangedBeforeParent: Type = WM_DPICHANGED_BEFOREPARENT;
	pub const DrawClipboard: Type = WM_DRAWCLIPBOARD;
	pub const DrawItem: Type = WM_DRAWITEM;
	pub const DropFiles: Type = WM_DROPFILES;
	pub const DwmColorizationColorChanged: Type = WM_DWMCOLORIZATIONCOLORCHANGED;
	pub const DwmCompositionChanged: Type = WM_DWMCOMPOSITIONCHANGED;
	pub const DwmncRenderingChanged: Type = WM_DWMNCRENDERINGCHANGED;
	pub const DwmSendIconicLivePreviewBitmap: Type = WM_DWMSENDICONICLIVEPREVIEWBITMAP;
	pub const DwmSendIconicThumbnail: Type = WM_DWMSENDICONICTHUMBNAIL;
	pub const DwmWindowMaximizedChange: Type = WM_DWMWINDOWMAXIMIZEDCHANGE;
	pub const Enable: Type = WM_ENABLE;
	pub const EndSession: Type = WM_ENDSESSION;
	pub const EnterIdle: Type = WM_ENTERIDLE;
	pub const EnterMenuLoop: Type = WM_ENTERMENULOOP;
	pub const EnterSizeMove: Type = WM_ENTERSIZEMOVE;
	pub const EraseBkgnd: Type = WM_ERASEBKGND;
	pub const ExitMenuLoop: Type = WM_EXITMENULOOP;
	pub const ExitSizeMove: Type = WM_EXITSIZEMOVE;
	pub const FontChange: Type = WM_FONTCHANGE;
	pub const Gesture: Type = WM_GESTURE;
	pub const GestureNotify: Type = WM_GESTURENOTIFY;
	pub const GetDlgCode: Type = WM_GETDLGCODE;
	pub const GetDpiScaledSize: Type = WM_GETDPISCALEDSIZE;
	pub const GetFont: Type = WM_GETFONT;
	pub const GetHotkey: Type = WM_GETHOTKEY;
	pub const GetIcon: Type = WM_GETICON;
	pub const GetMinmaxinfo: Type = WM_GETMINMAXINFO;
	pub const GetObject: Type = WM_GETOBJECT;
	pub const GetText: Type = WM_GETTEXT;
	pub const GetTextLength: Type = WM_GETTEXTLENGTH;
	pub const GetTitlebarInfoEx: Type = WM_GETTITLEBARINFOEX;
	pub const HandheldFirst: Type = WM_HANDHELDFIRST;
	pub const HandheldLast: Type = WM_HANDHELDLAST;
	pub const Help: Type = WM_HELP;
	pub const Hotkey: Type = WM_HOTKEY;
	pub const HScroll: Type = WM_HSCROLL;
	pub const HScrollClipboard: Type = WM_HSCROLLCLIPBOARD;
	pub const IconEraseBkgnd: Type = WM_ICONERASEBKGND;
	pub const Ime_char: Type = WM_IME_CHAR;
	pub const Ime_composition: Type = WM_IME_COMPOSITION;
	pub const Ime_compositionFull: Type = WM_IME_COMPOSITIONFULL;
	pub const Ime_control: Type = WM_IME_CONTROL;
	pub const Ime_endComposition: Type = WM_IME_ENDCOMPOSITION;
	pub const Ime_keyDown: Type = WM_IME_KEYDOWN;
	pub const Ime_keyLast: Type = WM_IME_KEYLAST;
	pub const Ime_keyUp: Type = WM_IME_KEYUP;
	pub const Ime_notify: Type = WM_IME_NOTIFY;
	pub const Ime_request: Type = WM_IME_REQUEST;
	pub const Ime_select: Type = WM_IME_SELECT;
	pub const Ime_setContext: Type = WM_IME_SETCONTEXT;
	pub const Ime_startComposition: Type = WM_IME_STARTCOMPOSITION;
	pub const InitDialog: Type = WM_INITDIALOG;
	pub const InitMenu: Type = WM_INITMENU;
	pub const InitMenuPopup: Type = WM_INITMENUPOPUP;
	pub const Input: Type = WM_INPUT;
	pub const InputLangChange: Type = WM_INPUTLANGCHANGE;
	pub const InputLangChangeRequest: Type = WM_INPUTLANGCHANGEREQUEST;
	pub const Input_device_change: Type = WM_INPUT_DEVICE_CHANGE;
	pub const KeyDown: Type = WM_KEYDOWN;
	pub const KeyFirst: Type = WM_KEYFIRST;
	pub const KeyLast: Type = WM_KEYLAST;
	pub const KeyUp: Type = WM_KEYUP;
	pub const KillFocus: Type = WM_KILLFOCUS;
	pub const LButtonDblclk: Type = WM_LBUTTONDBLCLK;
	pub const LButtonDown: Type = WM_LBUTTONDOWN;
	pub const LButtonUp: Type = WM_LBUTTONUP;
	pub const MButtonDblclk: Type = WM_MBUTTONDBLCLK;
	pub const MButtonDown: Type = WM_MBUTTONDOWN;
	pub const MButtonUp: Type = WM_MBUTTONUP;
	pub const MdiActivate: Type = WM_MDIACTIVATE;
	pub const MdiCascade: Type = WM_MDICASCADE;
	pub const MdiCreate: Type = WM_MDICREATE;
	pub const MdiDestroy: Type = WM_MDIDESTROY;
	pub const MdiGetActive: Type = WM_MDIGETACTIVE;
	pub const MdiIconArrange: Type = WM_MDIICONARRANGE;
	pub const MdiMaximize: Type = WM_MDIMAXIMIZE;
	pub const MdiNext: Type = WM_MDINEXT;
	pub const MdiRefreshMenu: Type = WM_MDIREFRESHMENU;
	pub const MdiRestore: Type = WM_MDIRESTORE;
	pub const MdiSetMenu: Type = WM_MDISETMENU;
	pub const MdiTile: Type = WM_MDITILE;
	pub const MeasureItem: Type = WM_MEASUREITEM;
	pub const MenuChar: Type = WM_MENUCHAR;
	pub const MenuCommand: Type = WM_MENUCOMMAND;
	pub const MenuDrag: Type = WM_MENUDRAG;
	pub const MenuGetObject: Type = WM_MENUGETOBJECT;
	pub const MenurButtonUp: Type = WM_MENURBUTTONUP;
	pub const MenuSelect: Type = WM_MENUSELECT;
	pub const MouseActivate: Type = WM_MOUSEACTIVATE;
	pub const MouseFirst: Type = WM_MOUSEFIRST;
	pub const MouseHWheel: Type = WM_MOUSEHWHEEL;
	pub const MouseLast: Type = WM_MOUSELAST;
	pub const MouseMove: Type = WM_MOUSEMOVE;
	pub const MouseWheel: Type = WM_MOUSEWHEEL;
	pub const Move: Type = WM_MOVE;
	pub const Moving: Type = WM_MOVING;
	pub const NcActivate: Type = WM_NCACTIVATE;
	pub const NcCalcSize: Type = WM_NCCALCSIZE;
	pub const NcCreate: Type = WM_NCCREATE;
	pub const NcDestroy: Type = WM_NCDESTROY;
	pub const NcHitTest: Type = WM_NCHITTEST;
	pub const NclButtonDblclk: Type = WM_NCLBUTTONDBLCLK;
	pub const NclButtonDown: Type = WM_NCLBUTTONDOWN;
	pub const NclButtonUp: Type = WM_NCLBUTTONUP;
	pub const NcmButtonDblclk: Type = WM_NCMBUTTONDBLCLK;
	pub const NcmButtonDown: Type = WM_NCMBUTTONDOWN;
	pub const NcmButtonUp: Type = WM_NCMBUTTONUP;
	pub const NcMouseHover: Type = WM_NCMOUSEHOVER;
	pub const NcMouseLeave: Type = WM_NCMOUSELEAVE;
	pub const NcMouseMove: Type = WM_NCMOUSEMOVE;
	pub const NcPaint: Type = WM_NCPAINT;
	pub const NcPointerDown: Type = WM_NCPOINTERDOWN;
	pub const NcPointerUp: Type = WM_NCPOINTERUP;
	pub const NcPointerUpdate: Type = WM_NCPOINTERUPDATE;
	pub const NcrButtonDblclk: Type = WM_NCRBUTTONDBLCLK;
	pub const NcrButtonDown: Type = WM_NCRBUTTONDOWN;
	pub const NcrButtonUp: Type = WM_NCRBUTTONUP;
	pub const NcxButtonDblclk: Type = WM_NCXBUTTONDBLCLK;
	pub const NcxButtonDown: Type = WM_NCXBUTTONDOWN;
	pub const NcxButtonUp: Type = WM_NCXBUTTONUP;
	pub const NextDlgctl: Type = WM_NEXTDLGCTL;
	pub const NextMenu: Type = WM_NEXTMENU;
	pub const NotifyFormat: Type = WM_NOTIFYFORMAT;
	pub const Null: Type = WM_NULL;
	pub const Paint: Type = WM_PAINT;
	pub const PaintClipboard: Type = WM_PAINTCLIPBOARD;
	pub const PaintIcon: Type = WM_PAINTICON;
	pub const PaletteChanged: Type = WM_PALETTECHANGED;
	pub const PaletteIsChanging: Type = WM_PALETTEISCHANGING;
	pub const ParentNotify: Type = WM_PARENTNOTIFY;
	pub const Paste: Type = WM_PASTE;
	pub const PenWinFirst: Type = WM_PENWINFIRST;
	pub const PenWinLast: Type = WM_PENWINLAST;
	pub const PointerActivate: Type = WM_POINTERACTIVATE;
	pub const PointerCaptureChanged: Type = WM_POINTERCAPTURECHANGED;
	pub const PointerDeviceChange: Type = WM_POINTERDEVICECHANGE;
	pub const PointerDeviceInRange: Type = WM_POINTERDEVICEINRANGE;

	// TODO: capitalize correctl 🔽
	pub const Pointerdeviceoutofrange: Type = WM_POINTERDEVICEOUTOFRANGE;
	pub const Pointerdown: Type = WM_POINTERDOWN;
	pub const Pointerenter: Type = WM_POINTERENTER;
	pub const Pointerhwheel: Type = WM_POINTERHWHEEL;
	pub const Pointerleave: Type = WM_POINTERLEAVE;
	pub const Pointerroutedaway: Type = WM_POINTERROUTEDAWAY;
	pub const Pointerroutedreleased: Type = WM_POINTERROUTEDRELEASED;
	pub const Pointerroutedto: Type = WM_POINTERROUTEDTO;
	pub const Pointerup: Type = WM_POINTERUP;
	pub const Pointerupdate: Type = WM_POINTERUPDATE;
	pub const Pointerwheel: Type = WM_POINTERWHEEL;
	pub const Power: Type = WM_POWER;
	pub const Powerbroadcast: Type = WM_POWERBROADCAST;
	pub const Print: Type = WM_PRINT;
	pub const Querydragicon: Type = WM_QUERYDRAGICON;
	pub const Queryendsession: Type = WM_QUERYENDSESSION;
	pub const Querynewpalette: Type = WM_QUERYNEWPALETTE;
	pub const Queryopen: Type = WM_QUERYOPEN;
	pub const Queryuistate: Type = WM_QUERYUISTATE;
	pub const Queuesync: Type = WM_QUEUESYNC;
	pub const Quit: Type = WM_QUIT;
	pub const Rbuttondblclk: Type = WM_RBUTTONDBLCLK;
	pub const Rbuttondown: Type = WM_RBUTTONDOWN;
	pub const Rbuttonup: Type = WM_RBUTTONUP;
	pub const Renderallformats: Type = WM_RENDERALLFORMATS;
	pub const Renderformat: Type = WM_RENDERFORMAT;
	pub const Setcursor: Type = WM_SETCURSOR;
	pub const Setfocus: Type = WM_SETFOCUS;
	pub const Setfont: Type = WM_SETFONT;
	pub const Sethotkey: Type = WM_SETHOTKEY;
	pub const Seticon: Type = WM_SETICON;
	pub const Setredraw: Type = WM_SETREDRAW;
	pub const Settext: Type = WM_SETTEXT;
	pub const Settingchange: Type = WM_SETTINGCHANGE;
	pub const Showwindow: Type = WM_SHOWWINDOW;
	pub const Size: Type = WM_SIZE;
	pub const Sizeclipboard: Type = WM_SIZECLIPBOARD;
	pub const Sizing: Type = WM_SIZING;
	pub const Spoolerstatus: Type = WM_SPOOLERSTATUS;
	pub const Stylechanged: Type = WM_STYLECHANGED;
	pub const Stylechanging: Type = WM_STYLECHANGING;
	pub const Syncpaint: Type = WM_SYNCPAINT;
	pub const Syschar: Type = WM_SYSCHAR;
	pub const Syscolorchange: Type = WM_SYSCOLORCHANGE;
	pub const Syscommand: Type = WM_SYSCOMMAND;
	pub const Sysdeadchar: Type = WM_SYSDEADCHAR;
	pub const Syskeydown: Type = WM_SYSKEYDOWN;
	pub const Syskeyup: Type = WM_SYSKEYUP;
	pub const Tablet_first: Type = WM_TABLET_FIRST;
	pub const Tablet_last: Type = WM_TABLET_LAST;
	pub const Tcard: Type = WM_TCARD;
	pub const Themechanged: Type = WM_THEMECHANGED;
	pub const Timechange: Type = WM_TIMECHANGE;
	pub const Timer: Type = WM_TIMER;
	pub const Touch: Type = WM_TOUCH;
	pub const Touchhittesting: Type = WM_TOUCHHITTESTING;
	pub const Undo: Type = WM_UNDO;
	pub const Uninitmenupopup: Type = WM_UNINITMENUPOPUP;
	pub const Updateuistate: Type = WM_UPDATEUISTATE;
	pub const User: Type = WM_USER;
	pub const Userchanged: Type = WM_USERCHANGED;
	pub const Vkeytoitem: Type = WM_VKEYTOITEM;
	pub const Vscroll: Type = WM_VSCROLL;
	pub const Vscrollclipboard: Type = WM_VSCROLLCLIPBOARD;
	pub const Windowposchanged: Type = WM_WINDOWPOSCHANGED;
	pub const Windowposchanging: Type = WM_WINDOWPOSCHANGING;
	pub const Wininichange: Type = WM_WININICHANGE;
	pub const Wtssession_change: Type = WM_WTSSESSION_CHANGE;
	pub const Xbuttondblclk: Type = WM_XBUTTONDBLCLK;
	pub const Xbuttondown: Type = WM_XBUTTONDOWN;
	pub const XBUTTONUP: Type = WM_XBUTTONUP;
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod show_cmd {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = SHOW_WINDOW_CMD;

	pub const ForceMinimize: Type = SW_FORCEMINIMIZE;
	pub const Hide: Type = SW_HIDE;
	pub const Maximize: Type = SW_MAXIMIZE;
	pub const Minimize: Type = SW_MINIMIZE;
	pub const Restore: Type = SW_RESTORE;
	pub const Show: Type = SW_SHOW;
	pub const ShowDefault: Type = SW_SHOWDEFAULT;
	pub const ShowMaximized: Type = SW_SHOWMAXIMIZED;
	pub const ShowMinimized: Type = SW_SHOWMINIMIZED;
	pub const ShowMinNoActive: Type = SW_SHOWMINNOACTIVE;
	pub const ShowNA: Type = SW_SHOWNA;
	pub const ShowNoActivate: Type = SW_SHOWNOACTIVATE;
	pub const ShowNormal: Type = SW_SHOWNORMAL;
	pub const Normal: Type = SW_NORMAL;
	pub const Max: Type = SW_MAX;
	pub const ParentClosing: Type = SW_PARENTCLOSING;
	pub const OtherZoom: Type = SW_OTHERZOOM;
	pub const ParentOpening: Type = SW_PARENTOPENING;
	pub const OtherUnzoom: Type = SW_OTHERUNZOOM;
	pub const ScrollChildren: Type = SW_SCROLLCHILDREN;
	pub const Invalidate: Type = SW_INVALIDATE;
	pub const Erase: Type = SW_ERASE;
	pub const SmoothScroll: Type = SW_SMOOTHSCROLL;
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod ex_style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub struct Type(pub WINDOW_EX_STYLE);

	pub const DlgModalFrame: Type = Type(WS_EX_DLGMODALFRAME);
	pub const NoParentNotify: Type = Type(WS_EX_NOPARENTNOTIFY);
	pub const TopMost: Type = Type(WS_EX_TOPMOST);
	pub const AcceptFiles: Type = Type(WS_EX_ACCEPTFILES);
	pub const Transparent: Type = Type(WS_EX_TRANSPARENT);
	pub const MdiChild: Type = Type(WS_EX_MDICHILD);
	pub const ToolWindow: Type = Type(WS_EX_TOOLWINDOW);
	pub const WindowEdge: Type = Type(WS_EX_WINDOWEDGE);
	pub const ClientEdge: Type = Type(WS_EX_CLIENTEDGE);
	pub const ContextHelp: Type = Type(WS_EX_CONTEXTHELP);
	pub const Right: Type = Type(WS_EX_RIGHT);
	pub const Left: Type = Type(WS_EX_LEFT);
	pub const RtlReading: Type = Type(WS_EX_RTLREADING);
	pub const LtrReading: Type = Type(WS_EX_LTRREADING);
	pub const LeftScrollbar: Type = Type(WS_EX_LEFTSCROLLBAR);
	pub const RightScrollbar: Type = Type(WS_EX_RIGHTSCROLLBAR);
	pub const ControlParent: Type = Type(WS_EX_CONTROLPARENT);
	pub const StaticEdge: Type = Type(WS_EX_STATICEDGE);
	pub const AppWindow: Type = Type(WS_EX_APPWINDOW);
	pub const OverlappedWindow: Type = Type(WS_EX_OVERLAPPEDWINDOW);
	pub const PaletteWindow: Type = Type(WS_EX_PALETTEWINDOW);
	pub const Layered: Type = Type(WS_EX_LAYERED);
	pub const NoInheritLayout: Type = Type(WS_EX_NOINHERITLAYOUT);
	pub const NoRedirectionBitmap: Type = Type(WS_EX_NOREDIRECTIONBITMAP);
	pub const LayoutRtl: Type = Type(WS_EX_LAYOUTRTL);
	pub const Composited: Type = Type(WS_EX_COMPOSITED);
	pub const NoActivate: Type = Type(WS_EX_NOACTIVATE);
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	#[derive(Copy, Clone)]
	pub struct Type(pub WINDOW_STYLE);

	pub const Overlapped: Type = Type(WS_OVERLAPPED);
	pub const Popup: Type = Type(WS_POPUP);
	pub const Child: Type = Type(WS_CHILD);
	pub const Minimize: Type = Type(WS_MINIMIZE);
	pub const Visible: Type = Type(WS_VISIBLE);
	pub const Disabled: Type = Type(WS_DISABLED);
	pub const ClipSiblings: Type = Type(WS_CLIPSIBLINGS);
	pub const ClipChildren: Type = Type(WS_CLIPCHILDREN);
	pub const Maximize: Type = Type(WS_MAXIMIZE);
	pub const Caption: Type = Type(WS_CAPTION);
	pub const Border: Type = Type(WS_BORDER);
	pub const DlgFrame: Type = Type(WS_DLGFRAME);
	pub const VScroll: Type = Type(WS_VSCROLL);
	pub const HScroll: Type = Type(WS_HSCROLL);
	pub const SysMenu: Type = Type(WS_SYSMENU);
	pub const ThickFrame: Type = Type(WS_THICKFRAME);
	pub const Group: Type = Type(WS_GROUP);
	pub const Tabstop: Type = Type(WS_TABSTOP);
	pub const MinimizeBox: Type = Type(WS_MINIMIZEBOX);
	pub const MaximizeBox: Type = Type(WS_MAXIMIZEBOX);
	pub const Tiled: Type = Type(WS_TILED);
	pub const Iconic: Type = Type(WS_ICONIC);
	pub const SizeBox: Type = Type(WS_SIZEBOX);
	pub const TiledWindow: Type = Type(WS_TILEDWINDOW);
	pub const OverlappedWindow: Type = Type(WS_OVERLAPPEDWINDOW);
	pub const PopupWindow: Type = Type(WS_POPUPWINDOW);
	pub const ChildWindow: Type = Type(WS_CHILDWINDOW);
	pub const ActiveCaption: Type = Type(WS_ACTIVECAPTION);
}
