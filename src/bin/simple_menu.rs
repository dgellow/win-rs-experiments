// Based on example "A simple menu" from https://zetcode.com/gui/winapi/menus/

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
		cursor, icon, menu, message_box,
		wide_string::ToWide,
		window::{class_style, ex_style, message, show_cmd, style, Point},
	};
	use windows::Win32::{
		Foundation::BOOL,
		Graphics::Gdi::{UpdateWindow, HBRUSH},
		UI::WindowsAndMessaging::{
			AppendMenuW, CreateMenu, CreatePopupMenu, CreateWindowExW, DefWindowProcW,
			DispatchMessageW, GetMessageW, LoadIconW, PostQuitMessage, RegisterClassExW, SetMenu,
			ShowWindow, TranslateMessage, COLOR_BACKGROUND, MSG,
		},
	};
	use windows::Win32::{
		Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
		UI::WindowsAndMessaging::WNDCLASSEXW,
	};
	use windows::Win32::{
		System::LibraryLoader::GetModuleHandleExW, UI::WindowsAndMessaging::LoadCursorW,
	};

	const CLASS_NAME: &str = "MainWindow";
	const TITLE: &str = "Simple menu — Win32 💖 Rust";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	const IDM_FILE_NEW: usize = 1;

	const IDM_IMPORT_MAIL: usize = 11;

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

		let icon = unsafe { LoadIconW(0, icon::Application) };
		assert_ne(icon, 0, "failed to get icon handle").with_last_win32_err()?;

		let cursor = unsafe { LoadCursorW(0, cursor::Arrow) };
		assert_ne(cursor, 0, "failed to get cursor handle").with_last_win32_err()?;

		let brush: HBRUSH = COLOR_BACKGROUND
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
		let dimension = Point { x: 500, y: 100 };

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
			let menu = CreateMenu();
			let submenu = CreatePopupMenu();

			let mut res = AppendMenuW(menu, menu::item_flag::String, IDM_FILE_NEW, "&New");
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = AppendMenuW(
				menu,
				menu::item_flag::String | menu::item_flag::Popup,
				submenu
					.try_into()
					.expect("failed to convert isize to usize"),
				"&Import",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = AppendMenuW(
				submenu,
				menu::item_flag::String,
				IDM_IMPORT_MAIL,
				"Import &mail",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = AppendMenuW(
				menubar,
				menu::item_flag::Popup,
				menu.try_into().expect("failed to convert isize to usize"),
				"&File",
			);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;

			res = SetMenu(window, menubar);
			assert_eq(res, BOOL(1), "failed to append menu").with_last_win32_err()?;
		}

		Ok(())
	}

	extern "system" fn win_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		match message {
			message::Create => create_menus(window).unwrap(),
			message::Command => match wparam {
				IDM_FILE_NEW => {
					message_box::new("New file selected", "Information", message_box::style::Ok)
						.unwrap();
				}
				IDM_IMPORT_MAIL => {
					message_box::new(
						"Import mail selected",
						"Information",
						message_box::style::Ok,
					)
					.unwrap();
				}
				_ => {}
			},
			message::Destroy => unsafe { PostQuitMessage(0) },
			_ => {}
		}
		unsafe { DefWindowProcW(window, message, wparam, lparam) }
	}
}
