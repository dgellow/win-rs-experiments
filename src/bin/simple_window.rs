use gui::assert::Result;

fn main() -> Result<()> {
	app()
}

fn app() -> Result<()> {
	child_window::init()?;

	main_window::init()?;
	main_window::create("My Window", Default::default(), Default::default())?;

	Ok(())
}

mod main_window {
	use gui::{
		assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
		display,
		wide_string::ToWide,
		window::{class_style, message, show_cmd, style, Point},
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
		Graphics::Gdi::ValidateRect,
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcA, EnumChildWindows, GetClientRect, GetWindowLongW,
			LoadCursorW, MoveWindow, PostQuitMessage, RegisterClassExW, ShowWindow, GWL_ID,
			WNDCLASSEXW,
		},
	};

	use crate::child_window;

	pub static CLASS_NAME: &str = "MainWindow";
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
		if unsafe { H_INSTANCE }.is_some() {
			Err("main_window already initialized".into())
		} else {
			Ok(())
		}
	}

	pub fn init() -> Result<()> {
		assert_not_init()?;

		display!("init main_window...");

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
			style: class_style::HRedraw | class_style::VRedraw,
			lpfnWndProc: Some(window_proc),
			hInstance: h_instance,
			hCursor: h_cursor,
			lpszClassName: "MyHappyMainWindow".to_wide().as_pwstr(), // CLASS_NAME.to_wide().as_pwstr(),
			..Default::default()
		};

		// register class
		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		display!("main_window initialized.");

		unsafe { H_INSTANCE = Some(h_instance) };

		Ok(())
	}

	pub fn create(title: &str, position: Point, dimension: Point) -> Result<()> {
		let h_instance = assert_init()?;

		display!("create main_window...");

		// create window
		let h_window = unsafe {
			CreateWindowExW(
				Default::default(),
				CLASS_NAME.to_wide().as_pwstr(),
				title.to_wide().as_pwstr(),
				style::OverlappedWindow | style::VScroll | style::HScroll,
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
		assert_ne(h_window, 0, "failed to create main window").with_last_win32_err()?;

		// show window
		assert_eq(
			unsafe { ShowWindow(h_window, show_cmd::Show) },
			BOOL(1),
			"failed to show main window",
		)
		.with_last_win32_err()?;

		display!("main_window created.");

		Ok(())
	}

	extern "system" fn window_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		// let h_instance = assert_init().unwrap();
		let mut rect: Box<RECT> = Box::new(Default::default());

		match message as message::Type {
			message::Paint => {
				println!("WM_PAINT");
				unsafe { ValidateRect(window, std::ptr::null()) };
				0
			}

			// creating main window
			message::Create => {
				println!("WM_CREATE");

				for i in 0..NB_CHILD {
					let child_id = CHILD_BASE_ID + i;
					display!("create child #{} ...", child_id);
					child_window::create(window, child_id).unwrap();
					display!("child #{} created.", child_id);
				}
				0
			}

			// main window changed size
			message::Size => {
				println!("WM_SIZE");
				unsafe { GetClientRect(window, rect.as_mut()) };
				let p: LPARAM = rect.as_mut() as *mut _ as _;
				unsafe { EnumChildWindows(window, Some(enum_child_proc), p) };
				0
			}

			message::Destroy => {
				println!("WM_DESTROY");
				unsafe { PostQuitMessage(0) };
				0
			}
			_ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
		}
	}

	extern "system" fn enum_child_proc(child: HWND, lparam: LPARAM) -> BOOL {
		display!("enumerate children");

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

		assert_eq(
			unsafe { ShowWindow(child, show_cmd::Show) },
			BOOL(1),
			"failed to show child window",
		)
		.with_last_win32_err()
		.unwrap();

		BOOL(1)
	}
}

mod child_window {
	use gui::{
		assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
		display,
		wide_string::ToWide,
		window::{class_style, show_cmd, style},
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, PWSTR},
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, LoadCursorW, RegisterClassExW, ShowWindow, HMENU, WNDCLASSEXW,
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

		display!("init child_window...");

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
			style: class_style::HRedraw | class_style::VRedraw,
			lpfnWndProc: None,
			hInstance: h_instance,
			hCursor: h_cursor,
			lpszClassName: "MyHappyChildClass".to_wide().as_pwstr(), // CLASS_NAME.to_wide().as_pwstr(),
			..Default::default()
		};

		// register class
		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne(class, 0, "failed to register class").with_last_win32_err()?;

		display!("child_window initialized.");

		unsafe { H_INSTANCE = Some(h_instance) };

		Ok(())
	}

	pub fn create(parent: HWND, child_id: i32) -> Result<()> {
		let h_instance = assert_init()?;

		let h_menu: HMENU = child_id.try_into().unwrap();
		let title: PWSTR = Default::default(); // null

		display!(
			"child_window::create: create window for child #{:#?} in parent window {:#?} with h_instance {:#?}...",
			child_id,
			parent,
			h_instance
		);

		let child = unsafe {
			CreateWindowExW(
				Default::default(),
				CLASS_NAME.to_wide().as_pwstr(),
				title,
				style::Child | style::Border,
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
		display!("child_window::create: window for child created.");

		assert_ne(child, 0, "failed to create child window").with_last_win32_err()?;

		display!("show child #{}...", child_id);

		assert_eq(
			unsafe { ShowWindow(child, show_cmd::Show) },
			BOOL(1),
			format!("failed to show child window #{}", child_id).as_str(),
		)
		.with_last_win32_err()?;

		display!("child #{} shown.", child_id);

		Ok(())
	}
}
