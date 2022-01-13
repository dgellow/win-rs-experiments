use gui::{assert::Result, display, err_display};

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
	child_window::init()?;
	main_window::init()?;
	main_window::create()?;
	let res = main_window::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

mod main_window {
	use gui::{
		assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
		cursor::{self, load_cursor},
		display,
		icon::{self, load_icon},
		wide_string::ToWide,
		window::{class_style, ex_style, message, show_cmd, style},
		Point,
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
		Graphics::Gdi::{UpdateWindow, HBRUSH},
		System::LibraryLoader::GetModuleHandleExW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcW, DispatchMessageW, EnumChildWindows, GetClientRect,
			GetMessageW, GetWindowLongW, MoveWindow, PostQuitMessage, RegisterClassExW, ShowWindow,
			TranslateMessage, COLOR_WINDOW, GWL_ID, MSG, WNDCLASSEXW,
		},
	};

	use crate::child_window;

	const CLASS_NAME: &str = "MainWindow";
	const TITLE: &str = "Empty Window — Win32 💖 Rust";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	const NB_CHILD: i32 = 3;
	const CHILD_BASE_ID: i32 = 100;

	fn assert_init() -> Result<HINSTANCE> {
		match unsafe { H_INSTANCE } {
			Some(instance) => Ok(instance),
			None => Err("main_window not initialized".into()),
		}
	}

	fn assert_not_init() -> Result<()> {
		match unsafe { H_INSTANCE } {
			Some(_) => Err("main_window already initialized".into()),
			None => Ok(()),
		}
	}

	pub fn init() -> Result<()> {
		assert_not_init()?;

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
			lpfnWndProc: Some(win_proc),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: h_instance,
			hIcon: icon,
			hCursor: cursor,
			hbrBackground: brush,
			lpszMenuName: Default::default(), // defaults to null
			lpszClassName: CLASS_NAME.to_wide().as_pwstr(),
			hIconSm: icon,
		};

		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		unsafe { H_INSTANCE = Some(h_instance) };

		Ok(())
	}

	pub fn create() -> Result<()> {
		let h_instance = assert_init()?;

		let position: Point = Default::default();
		let dimension = Point { x: 500, y: 400 };

		let h_window = unsafe {
			CreateWindowExW(
				ex_style::OverlappedWindow.0,
				CLASS_NAME.to_wide().as_pwstr(),
				TITLE.to_wide().as_pwstr(),
				style::OverlappedWindow.0,
				position.x,
				position.y,
				dimension.x,
				dimension.y,
				None,
				None,
				h_instance,
				std::ptr::null(),
			)
		};
		assert_ne(h_window, 0, "failed to create window").with_last_win32_err()?;

		unsafe { ShowWindow(h_window, show_cmd::Show) };
		unsafe { UpdateWindow(h_window) };

		Ok(())
	}

	pub fn event_loop() -> usize {
		let mut msg: MSG = Default::default();
		let msg_ptr = std::ptr::addr_of_mut!(msg);

		while unsafe { GetMessageW(msg_ptr, 0, 0, 0) }.as_bool() {
			unsafe { TranslateMessage(msg_ptr) };
			unsafe { DispatchMessageW(msg_ptr) };
		}

		unsafe { *msg_ptr }.wParam
	}

	extern "system" fn win_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		let _h_instance = assert_init().unwrap();

		match message {
			message::Paint => display!("WM_PAINT"),
			message::Create => {
				display!("WM_CREATE");
				for i in 0..NB_CHILD {
					let child_id = CHILD_BASE_ID + i;
					child_window::create(window, child_id).unwrap();
				}
			}
			message::Size => {
				display!("WM_SIZE");

				let mut rect: RECT = Default::default();
				unsafe {
					GetClientRect(window, &mut rect as *mut _);
					EnumChildWindows(window, Some(enum_child_proc), &mut rect as *mut _ as LPARAM);
				}
			}
			message::Destroy => {
				display!("WM_DESTROY");
				unsafe { PostQuitMessage(0) };
			}
			_ => {}
		}
		unsafe { DefWindowProcW(window, message, wparam, lparam) }
	}

	extern "system" fn enum_child_proc(child: HWND, lparam: LPARAM) -> BOOL {
		let id_child = unsafe { GetWindowLongW(child, GWL_ID) };
		let idx_child = id_child - CHILD_BASE_ID;

		let parent = lparam as *mut RECT;
		unsafe {
			MoveWindow(
				child,
				((*parent).right / 3) * idx_child,
				0,
				(*parent).right / 3,
				(*parent).bottom,
				BOOL(1),
			)
		};

		unsafe { ShowWindow(child, show_cmd::Show) };

		BOOL(1)
	}
}

mod child_window {
	use gui::{
		assert::{assert_ne, Result, WithLastWin32Error},
		wide_string::ToWide,
		window::{class_style, message, show_cmd, style},
	};
	use windows::Win32::{
		Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcW, LoadCursorW, RegisterClassExW, ShowWindow, HMENU,
			WNDCLASSEXW,
		},
	};

	pub static CLASS_NAME: &str = "ChildWindow";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	fn assert_init() -> Result<HINSTANCE> {
		match unsafe { H_INSTANCE } {
			Some(instance) => Ok(instance),
			None => Err("child_window not initialized".into()),
		}
	}

	fn assert_not_init() -> Result<()> {
		match unsafe { H_INSTANCE } {
			Some(_) => Err("child_window already initialized".into()),
			None => Ok(()),
		}
	}

	pub fn init() -> Result<()> {
		assert_not_init()?;

		// get instance handle
		let h_instance = unsafe { GetModuleHandleW(None) };
		assert_ne(h_instance, 0, "failed to get module handle").with_last_win32_err()?;

		// define class
		let h_cursor = unsafe { LoadCursorW(0, gui::cursor::Arrow) };
		assert_ne(h_cursor, 0, "failed to get cursor handle").with_last_win32_err()?;

		let size: u32 = std::mem::size_of::<WNDCLASSEXW>()
			.try_into()
			.expect("size of WNDCLASSEXW not u32");

		let wnd_class = WNDCLASSEXW {
			cbSize: size,
			style: class_style::HRedraw.0 | class_style::VRedraw.0,
			lpfnWndProc: Some(win_proc),
			hInstance: h_instance,
			hCursor: h_cursor,
			lpszClassName: CLASS_NAME.to_wide().as_pwstr(),
			..Default::default()
		};

		// register class
		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		unsafe { H_INSTANCE = Some(h_instance) };

		Ok(())
	}

	pub fn create(parent: HWND, child_id: i32) -> Result<()> {
		let h_instance = assert_init()?;

		let h_menu: HMENU = child_id.try_into().unwrap();
		let null_title: PWSTR = Default::default(); // defaults to null

		let child = unsafe {
			CreateWindowExW(
				Default::default(),
				CLASS_NAME.to_wide().as_pwstr(),
				null_title,
				(style::Child | style::Border).0,
				0,
				0,
				0,
				0,
				parent,
				h_menu,
				h_instance,
				std::ptr::null(),
			)
		};
		assert_ne(child, 0, "failed to create child window").with_last_win32_err()?;

		unsafe { ShowWindow(child, show_cmd::Show) };

		Ok(())
	}

	extern "system" fn win_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		let _h_instance = assert_init().unwrap();
		unsafe { DefWindowProcW(window, message, wparam, lparam) }
	}
}
