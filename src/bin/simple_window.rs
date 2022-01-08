use windows::core::Result;

fn main() -> Result<()> {
	child_window::init();

	main_window::init();
	main_window::create("My Window", Default::default(), Default::default());

	Ok(())
}

mod main_window {
	use gui::{
		display,
		wide_string::ToWide,
		window::{class_style, message, show_cmd, style, Point},
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM},
		Graphics::Gdi::ValidateRect,
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcA, EnumChildWindows, GetClientRect, GetWindowLongW,
			LoadCursorW, MoveWindow, PostQuitMessage, RegisterClassExW, ShowWindow, GWL_ID, HMENU,
			WNDCLASSEXW,
		},
	};

	use crate::child_window;

	pub static CLASS_NAME: &str = "MainWindow";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	const ID_CHILD_1: i32 = 100;
	const ID_CHILD_2: i32 = ID_CHILD_1 + 1;
	const ID_CHILD_3: i32 = ID_CHILD_1 + 2;

	fn assert_init() -> HINSTANCE {
		unsafe { assert!(H_INSTANCE.is_some(), "main_window not initialized") };
		unsafe { H_INSTANCE.unwrap() }
	}

	pub fn init() {
		// check if already initialized
		unsafe { assert!(H_INSTANCE.is_none(), "main_window already initialized") };

		display!("init main_window...");

		// get instance handle
		let h_instance = unsafe { GetModuleHandleW(None) };
		assert_ne!(h_instance, 0, "failed to get module handle");

		// define class
		let h_cursor = unsafe { LoadCursorW(0, gui::cursor::Arrow) };
		assert_ne!(h_cursor, 0, "failed to get cursor handle");

		let size: u32 = std::mem::size_of::<WNDCLASSEXW>()
			.try_into()
			.expect("size of WNDCLASSEXW not u32");

		let wnd_class = WNDCLASSEXW {
			cbSize: size,
			style: class_style::HRedraw | class_style::VRedraw,
			lpfnWndProc: Some(window_proc),
			hInstance: h_instance,
			hCursor: h_cursor,
			lpszClassName: CLASS_NAME.to_wide().as_pwstr(),
			..Default::default()
		};

		// register class
		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne!(class, 0, "failed to register class");

		display!("main_window initialized.");

		unsafe { H_INSTANCE = Some(h_instance) };
	}

	pub fn create(title: &str, position: Point, dimension: Point) {
		let h_instance = assert_init();

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
		assert_ne!(h_window, 0, "failed to create main window");

		// show window
		assert!(
			unsafe { ShowWindow(h_window, show_cmd::Show) }.as_bool(),
			"failed to show main window"
		);

		display!("main_window created.");
	}

	extern "system" fn window_proc(
		window: HWND,
		message: message::Type,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> LRESULT {
		let h_instance = assert_init();
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

				for i in 0..3 {
					display!("create child #{}", i + 1);
					let h_menu: HMENU = (ID_CHILD_1 + i).try_into().unwrap();
					let title: PWSTR = Default::default(); // null

					let class_name = child_window::CLASS_NAME.to_wide();
					let child = unsafe {
						CreateWindowExW(
							Default::default(),
							class_name.as_pwstr(),
							title,
							style::Child | style::Border,
							0,
							0,
							0,
							0,
							None,
							h_menu,
							h_instance,
							std::ptr::null(),
						)
					};

					assert!(
						unsafe { ShowWindow(child, show_cmd::Show) }.as_bool(),
						"failed to show child window #{}",
						i + 1,
					);
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

		let i = match id_child {
			ID_CHILD_1 => 0,
			ID_CHILD_2 => 1,
			ID_CHILD_3 => 2,
			unknown => panic!("unexpected child id: {}", unknown),
		};

		let parent = lparam as *mut RECT;
		unsafe {
			MoveWindow(
				child,
				((*parent).right / 3) * i,
				0,
				(*parent).right / 3,
				(*parent).bottom,
				BOOL(1),
			)
		};

		assert!(
			unsafe { ShowWindow(child, show_cmd::Show) }.as_bool(),
			"failed to show child window"
		);

		BOOL(1)
	}
}

mod child_window {
	use gui::{display, wide_string::ToWide, window::class_style};
	use windows::Win32::{
		Foundation::HINSTANCE,
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::{LoadCursorW, RegisterClassExW, WNDCLASSEXW},
	};

	pub static CLASS_NAME: &str = "ChildWindow";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	pub fn init() {
		// check if already initialized
		unsafe { assert!(H_INSTANCE.is_none(), "main_window already initialized") };

		display!("init child_window...");

		// get instance handle
		let h_instance = unsafe { GetModuleHandleW(None) };
		assert_ne!(h_instance, 0, "failed to get module handle");

		// define class
		let h_cursor = unsafe { LoadCursorW(0, gui::cursor::Arrow) };
		assert_ne!(h_cursor, 0, "failed to get cursor handle");

		let size: u32 = std::mem::size_of::<WNDCLASSEXW>()
			.try_into()
			.expect("size of WNDCLASSEXW not u32");

		let wnd_class = WNDCLASSEXW {
			cbSize: size,
			style: class_style::HRedraw | class_style::VRedraw,
			lpfnWndProc: None,
			hInstance: h_instance,
			hCursor: h_cursor,
			lpszClassName: CLASS_NAME.to_wide().as_pwstr(),
			..Default::default()
		};

		// register class
		let class = unsafe { RegisterClassExW(&wnd_class) };
		assert_ne!(class, 0, "failed to register class");

		display!("child_window initialized.");

		unsafe { H_INSTANCE = Some(h_instance) };
	}
}
