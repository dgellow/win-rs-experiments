use crate::wide_string::ToWide;

use std::sync::Once;
use windows::{
	core::Result,
	Win32::{
		Foundation::{HWND, LPARAM, LRESULT, WPARAM},
		Graphics::Gdi::UpdateWindow,
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::*,
	},
};

static REGISTER_WINDOW_CLASS: Once = Once::new();

pub struct Window {}

impl Window {
	fn new_impl(
		title: &str,
		position: Point,
		dimension: Point,
		window_proc: Option<WindowProc>,
	) -> Result<Box<Self>> {
		let class_name = "my window class".to_wide();
		let w_title = title.to_wide();

		// get the instance handle
		let instance = unsafe { GetModuleHandleW(None) };
		debug_assert_ne!(instance, 0, "failed to get module handle");

		REGISTER_WINDOW_CLASS.call_once(|| {
			// define a new class for the window
			use crate::class;
			let class = WNDCLASSW {
				hCursor: unsafe { LoadCursorW(0, crate::cursor::Arrow) },
				hInstance: instance,
				lpszClassName: class_name.as_pwstr(),
				style: class::style::HRedraw | class::style::VRedraw,
				lpfnWndProc: window_proc,
				..Default::default()
			};

			// create the window class
			let registered = unsafe { RegisterClassW(&class) };
			debug_assert_ne!(registered, 0, "failed to register class");
		});

		let mut result = Box::new(Self {});

		// create window
		let window = unsafe {
			CreateWindowExW(
				Default::default(),                                        // extended styles
				class_name.as_pwstr(),                                     // window class
				w_title.as_pwstr(),                                        // title
				style::OverlappedWindow | style::VScroll | style::HScroll, // style
				position.x,                                                // pos x
				position.y,                                                // pos y
				dimension.x,                                               // width
				dimension.y,                                               // height
				None,                                                      // parent window
				None,                                                      // menu used
				instance,                                                  // instance handle
				result.as_mut() as *mut _ as _,                            // window creation data
			)
		};
		debug_assert_ne!(window, 0, "failed to create window");

		// show the window
		unsafe { ShowWindow(window, show_cmd::ShowDefault) };
		// send WM_PAINT message to the window (handled by window_proc set in the window class)
		unsafe { UpdateWindow(window) };

		Ok(result)
	}

	pub fn new(title: &str, position: Point, dimension: Point) -> Result<Box<Self>> {
		Self::new_impl(title, position, dimension, None)
	}

	pub fn new_with_proc(
		title: &str,
		position: Point,
		dimension: Point,
		window_proc: WindowProc,
	) -> Result<Box<Self>> {
		Self::new_impl(title, position, dimension, Some(window_proc))
	}

	pub fn handle_events(&self) {
		let mut message = MSG::default();
		while unsafe { GetMessageW(&mut message, 0, 0, 0) }.into() {
			unsafe { DispatchMessageW(&mut message) };
		}
	}
}

pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Default for Point {
	fn default() -> Self {
		Self {
			x: CW_USEDEFAULT,
			y: CW_USEDEFAULT,
		}
	}
}

pub type WindowProc =
	unsafe extern "system" fn(window: HWND, message: message::Type, WPARAM, LPARAM) -> LRESULT;

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
mod show_cmd {
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

pub mod ex_style {}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	use windows::Win32::UI::WindowsAndMessaging::*;
	pub type Type = WINDOW_STYLE;

	pub const Overlapped: Type = WS_OVERLAPPED;
	pub const Popup: Type = WS_POPUP;
	pub const Child: Type = WS_CHILD;
	pub const Minimize: Type = WS_MINIMIZE;
	pub const Visible: Type = WS_VISIBLE;
	pub const Disabled: Type = WS_DISABLED;
	pub const ClipSiblings: Type = WS_CLIPSIBLINGS;
	pub const ClipChildren: Type = WS_CLIPCHILDREN;
	pub const Maximize: Type = WS_MAXIMIZE;
	pub const Caption: Type = WS_CAPTION;
	pub const Border: Type = WS_BORDER;
	pub const DlgFrame: Type = WS_DLGFRAME;
	pub const VScroll: Type = WS_VSCROLL;
	pub const HScroll: Type = WS_HSCROLL;
	pub const SysMenu: Type = WS_SYSMENU;
	pub const ThickFrame: Type = WS_THICKFRAME;
	pub const Group: Type = WS_GROUP;
	pub const Tabstop: Type = WS_TABSTOP;
	pub const MinimizeBox: Type = WS_MINIMIZEBOX;
	pub const MaximizeBox: Type = WS_MAXIMIZEBOX;
	pub const Tiled: Type = WS_TILED;
	pub const Iconic: Type = WS_ICONIC;
	pub const SizeBox: Type = WS_SIZEBOX;
	pub const TiledWindow: Type = WS_TILEDWINDOW;
	pub const OverlappedWindow: Type = WS_OVERLAPPEDWINDOW;
	pub const PopupWindow: Type = WS_POPUPWINDOW;
	pub const ChildWindow: Type = WS_CHILDWINDOW;
	pub const ActiveCaption: Type = WS_ACTIVECAPTION;
}
