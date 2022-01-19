use gui::{
	assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
	cursor::{self, load_cursor},
	display, err_display,
	icon::{self, load_icon},
	wide_string::ToWide,
	window::{class_style, ex_style, message, show_cmd, style},
	window_long::{get_window_long_ptr, set_window_long_ptr},
	Point,
};
use windows::Win32::{
	Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
	Graphics::Gdi::{UpdateWindow, HBRUSH},
	System::LibraryLoader::GetModuleHandleExW,
	UI::WindowsAndMessaging::{
		CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
		RegisterClassExW, ShowWindow, TranslateMessage, COLOR_WINDOW, CREATESTRUCTW, GWLP_USERDATA,
		MSG, WNDCLASSEXW,
	},
};

fn main() -> std::result::Result<(), ()> {
	match app() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

fn app() -> Result<()> {
	let main_window = MainWindow::new("MainWindow", "Extension Trait Window — Win32 💖 Rust")?;
	display!("main_window: {:?}", main_window);

	let res = MainWindow::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

enum MessageAction {
	Continue,
	_FullyHandled,
}

// 1. create our window type
#[derive(Debug)]
struct MainWindow {
	h_instance: HINSTANCE,
}

// 2. implement WindowBase trait
impl WindowBase for MainWindow {
	fn init_state(h_instance: HINSTANCE) -> Self {
		Self { h_instance }
	}

	fn h_instance(&self) -> HINSTANCE {
		self.h_instance
	}

	fn on_message(
		&self,
		_h_window: HWND,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> MessageAction {
		use MessageAction::*;

		match message {
			message::Paint => {
				display!("WM_PAINT");
				// unsafe { ValidateRect(h_window, std::ptr::null()) };
				Continue
			}
			message::MButtonDown => {
				display!("WM_MBUTTONDOWN");
				Continue
			}
			_ => Continue,
		}
	}
}

// 3. define functions required to initiate the state and handle messages as a WindowBas trait
trait WindowBase {
	fn init_state(h_instance: HINSTANCE) -> Self;
	fn h_instance(&self) -> HINSTANCE;
	fn on_message(
		&self,
		h_window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> MessageAction;
}

// 4. define an extension trait to implement win32 boilerplate:
// - win_proc
// - create and register class referencing win_proc
// - instantiate state
// - create window and pass state to WM_CREATE
// - event_loop
trait WindowBaseExt: WindowBase
where
	Self: Sized,
{
	fn new(class_name: &str, title: &str) -> Result<Self>
	where
		Self: Sized,
	{
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

		let icon = load_icon(icon::Application)?;
		let cursor = load_cursor(cursor::Arrow)?;

		let brush: HBRUSH = (COLOR_WINDOW + 1)
			.try_into()
			.expect("cannot convert color to HBRUSH");

		let wnd_class = WNDCLASSEXW {
			cbSize: size,
			style: class_style::HRedraw.0 | class_style::VRedraw.0,
			lpfnWndProc: Some(Self::win_proc), // 👈
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: h_instance,
			hIcon: icon,
			hCursor: cursor,
			hbrBackground: brush,
			lpszMenuName: Default::default(), // defaults to null
			lpszClassName: class_name.to_wide().as_pwstr(),
			hIconSm: icon,
		};

		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		let position: Point = Default::default();
		let dimension = Point { x: 500, y: 400 };

		let mut state = Self::init_state(h_instance);

		let h_window = unsafe {
			CreateWindowExW(
				ex_style::OverlappedWindow.0,
				class_name.to_wide().as_pwstr(),
				title.to_wide().as_pwstr(),
				style::OverlappedWindow.0,
				position.x,
				position.y,
				dimension.x,
				dimension.y,
				None,
				None,
				h_instance,
				&mut state as *mut _ as _,
			)
		};
		assert_ne(h_window, 0, "failed to create window").with_last_win32_err()?;

		unsafe { ShowWindow(h_window, show_cmd::Show) };
		unsafe { UpdateWindow(h_window) };

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

	extern "system" fn win_proc(
		h_window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		use MessageAction::*;
		unsafe {
			let state: *mut Self = match message {
				message::Destroy => {
					display!("WM_DESTROY");
					PostQuitMessage(0);
					std::ptr::null_mut()
				}
				message::Create => {
					display!("WM_CREATE");

					let create_struct = lparam as *mut CREATESTRUCTW;
					let state = (*create_struct).lpCreateParams as *mut Self;
					set_window_long_ptr(h_window, GWLP_USERDATA, state as _).unwrap();
					state
				}
				_ => get_window_long_ptr(h_window, GWLP_USERDATA).unwrap() as *mut _,
			};

			let default_win_proc = || DefWindowProcW(h_window, message, wparam, lparam);
			if state.is_null() {
				return default_win_proc();
			}

			match { (*state).on_message(h_window, message, wparam, lparam) } {
				Continue => default_win_proc(),
				_FullyHandled => 0,
			}
		}
	}
}

// 5. implement WindowBaseExt for all types implementing WindowBase
impl<T: WindowBase> WindowBaseExt for T {}
