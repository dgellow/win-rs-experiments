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
		menu, rich_edit,
		wide_string::ToWide,
		window::{class_style, ex_style, message, show_cmd, style},
		Point,
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
		Graphics::Gdi::{UpdateWindow, ValidateRect, HBRUSH},
		System::LibraryLoader::GetModuleHandleExW,
		UI::WindowsAndMessaging::{
			AppendMenuW, CreateMenu, CreateWindowExW, DefWindowProcW, DispatchMessageW,
			EnumChildWindows, GetClientRect, GetMessageW, MoveWindow, PostQuitMessage,
			RegisterClassExW, SetMenu, ShowWindow, TranslateMessage, COLOR_WINDOW, MSG,
			WNDCLASSEXW,
		},
	};

	const CLASS_NAME: &str = "MainWindow";
	const TITLE: &str = "Text Editor — Win32 💖 Rust";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

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
				ex_style::OverlappedWindow,
				CLASS_NAME.to_wide().as_pwstr(),
				TITLE.to_wide().as_pwstr(),
				style::OverlappedWindow,
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

	fn create_menus(window: HWND) -> Result<()> {
		unsafe {
			let menubar = CreateMenu();
			let menu_file = CreateMenu();
			let menu_edit = CreateMenu();
			let menu_view = CreateMenu();

			// menu bar:
			// | File | Edit | View |
			let mut res = AppendMenuW(
				menubar,
				menu::item_flag::Popup,
				menu_file
					.try_into()
					.expect("failed to convert menu to usize"),
				"&File",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = AppendMenuW(
				menubar,
				menu::item_flag::Popup,
				menu_edit
					.try_into()
					.expect("failed to convert menu to usize"),
				"&Edit",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = AppendMenuW(
				menubar,
				menu::item_flag::Popup,
				menu_view
					.try_into()
					.expect("failed to convert menu to usize"),
				"&View",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			// set window menu
			res = SetMenu(window, menubar);
			assert_eq(res, BOOL(1), "failed to set window menu").with_last_win32_err()?;
		}

		Ok(())
	}

	extern "system" fn win_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		let h_instance = assert_init().unwrap();

		match message {
			message::Paint => {
				unsafe { ValidateRect(window, std::ptr::null()) };
			}
			message::Create => {
				display!("WM_CREATE");

				create_menus(window).unwrap();

				let position: Point = Default::default();
				let dimension = Point { x: 200, y: 200 };
				rich_edit::create(window, h_instance, position, dimension).unwrap();
			}
			message::Size => {
				display!("WM_SIZE");

				let mut rect: RECT = Default::default();
				unsafe {
					GetClientRect(window, &mut rect as *mut _);
					EnumChildWindows(window, Some(resize_controls), &mut rect as *mut _ as LPARAM);
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

	extern "system" fn resize_controls(current: HWND, lparam: LPARAM) -> BOOL {
		let vertical_margin: i32 = 10;
		let horizontal_margin: i32 = 10;
		let parent = unsafe { *(lparam as *mut RECT) };
		unsafe {
			MoveWindow(
				current,
				parent.left + vertical_margin,
				parent.top + horizontal_margin,
				parent.right - vertical_margin,
				parent.bottom - horizontal_margin,
				BOOL(1),
			);
		};

		BOOL(1)
	}
}
