use gui::{
	assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
	cursor::{self, load_cursor},
	display, err_display,
	icon::{self, load_icon},
	wide_string::ToWide,
	window::{class_style, ex_style, message, show_cmd, style},
	Point,
};
use windows::Win32::{
	Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
	Graphics::Gdi::{UpdateWindow, ValidateRect, HBRUSH},
	System::LibraryLoader::GetModuleHandleExW,
	UI::WindowsAndMessaging::{
		CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
		PostQuitMessage, RegisterClassExW, SetWindowLongPtrW, ShowWindow, TranslateMessage,
		COLOR_WINDOW, CREATESTRUCTW, GWLP_USERDATA, MSG, WNDCLASSEXW,
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
	let main_window = Window::new("MainWindow", "OOP Window — Win32 💖 Rust")?;
	display!("main_window: {:?}", main_window);

	let res = Window::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

enum MessageAction {
	Continue,
	_FullyHandled,
}

// 1. define window type
#[derive(Debug)]
struct Window {
	_h_instance: HINSTANCE,
	h_window: HWND,
}

impl Window {
	// 2. constructor to register class and create window
	pub fn new(class_name: &str, title: &str) -> Result<Window> {
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
			lpfnWndProc: Some(Window::win_proc),
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

		// 3. create object we will pass as CreateWindow creation parameter.
		//
		// 📜 use Box<> type if it should be heap-allocated.
		let mut state = Window {
			_h_instance: h_instance,
			h_window: 0, // will be set after call to CreateWindow
		};

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
				// 4. pass object pointer as creation parameter
				&mut state as *mut _ as _,
			)
		};
		assert_ne(h_window, 0, "failed to create window").with_last_win32_err()?;

		// 5. save window handle into our object
		state.h_window = h_window;

		unsafe { ShowWindow(h_window, show_cmd::Show) };
		unsafe { UpdateWindow(h_window) };

		// 6. return object, passing ownership to the caller
		Ok(state)
	}

	// 7. handler function for incoming messages
	pub fn on_message(
		&self,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> MessageAction {
		use MessageAction::*;

		// 8. react to messages
		match message {
			message::Paint => {
				display!("WM_PAINT");
				unsafe { ValidateRect(self.h_window, std::ptr::null()) };
				Continue
			}
			_ => Continue,
		}
	}

	pub fn event_loop() -> usize {
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

	// 9. win_proc defined as static function on window type
	extern "system" fn win_proc(
		h_window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		use MessageAction::*;
		unsafe {
			// 10. extract state, from lparam on WM_CREATE message, otherwise from h_window USERDATA
			let state: *mut Window = match message {
				message::Destroy => {
					display!("WM_DESTROY");
					PostQuitMessage(0);
					std::ptr::null_mut()
				}
				message::Create => {
					display!("WM_CREATE");
					let create_struct = lparam as *mut CREATESTRUCTW;
					let state = (*create_struct).lpCreateParams as *mut Window;
					SetWindowLongPtrW(h_window, GWLP_USERDATA, state as _);
					state
				}
				_ => GetWindowLongPtrW(h_window, GWLP_USERDATA) as *mut _,
			};

			let default_win_proc = || DefWindowProcW(h_window, message, wparam, lparam);
			// 11. always check for null before dereferencing pointer
			if state.is_null() {
				return default_win_proc();
			}
			// 12. pass message to object method
			match { (*state).on_message(message, wparam, lparam) } {
				Continue => default_win_proc(),
				_FullyHandled => 0,
			}
		}
	}
}
