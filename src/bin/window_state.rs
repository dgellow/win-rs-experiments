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
	let state_info = main_window::create()?;
	display!("state_info after creation: {:?}", state_info);
	let res = main_window::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

mod main_window {
	use gui::{
		assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
		cursor, display, icon,
		wide_string::ToWide,
		window::{class_style, ex_style, message, show_cmd, style, Point},
	};
	use windows::Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
		Graphics::Gdi::{UpdateWindow, ValidateRect, HBRUSH},
		System::LibraryLoader::GetModuleHandleExW,
		UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
			LoadCursorW, LoadIconW, PostQuitMessage, RegisterClassExW, SetWindowLongPtrW,
			ShowWindow, TranslateMessage, COLOR_WINDOW, CREATESTRUCTW, GWLP_USERDATA, MSG,
			WNDCLASSEXW,
		},
	};

	const CLASS_NAME: &str = "MainWindow";
	const TITLE: &str = "Empty Window — Win32 💖 Rust";
	static mut H_INSTANCE: Option<HINSTANCE> = None;

	#[derive(Debug)]
	pub struct StateInfo {
		info: String,
	}

	impl Drop for StateInfo {
		fn drop(&mut self) {
			display!("StateInfo dropped");
		}
	}

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

	pub fn create() -> Result<Box<StateInfo>> {
		let h_instance = assert_init()?;

		let position: Point = Default::default();
		let dimension = Point { x: 500, y: 400 };

		// 1. allocate data to pass to the win_proc during WM_CREATE message
		let mut state_info = Box::<StateInfo>::new(StateInfo {
			info: "hello".to_owned(),
		});

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
				// 2. pass data to the CreateWindow function
				state_info.as_mut() as *mut _ as _,
			)
		};
		assert_ne(h_window, 0, "failed to create window").with_last_win32_err()?;

		unsafe { ShowWindow(h_window, show_cmd::Show) };
		unsafe { UpdateWindow(h_window) };

		display!("state_info once created: {:?}", state_info);

		// 3. return boxed data to extend its lifetime after
		//    the scope of the create function.
		Ok(state_info)
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

		let state_info: *mut StateInfo;
		if message == message::Create {
			unsafe {
				display!("WM_CREATE");
				// 4. lparam of VM_CREATE message is a pointer to CREATESTRUCT
				let create_struct = lparam as *mut CREATESTRUCTW;

				// 5. get passed data from lpCreateParams
				state_info = (*create_struct).lpCreateParams as *mut StateInfo;
				display!("state_info during creation: {:?}", *state_info);

				// 6. set window USER_DATA to our data pointer
				SetWindowLongPtrW(window, GWLP_USERDATA, state_info as _);

				// 7. modify data
				(*state_info).info = "how are you?".to_owned();
			}
		} else {
			// 8. get our data pointer from window USER_DATA
			let ptr = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) };
			state_info = ptr as *mut _;
		}

		// 9. before dereferencing, ensure we have a pointer to actual data
		if !state_info.is_null() {
			unsafe {
				// 10. use our data
				display!("state_info from win_proc: {:?}", *state_info);
			}
		}

		match message {
			message::Paint => {
				display!("WM_PAINT");
				unsafe { ValidateRect(window, std::ptr::null()) };
			}
			message::Destroy => {
				display!("WM_DESTROY");
				unsafe { PostQuitMessage(0) };
			}
			_ => {}
		}
		unsafe { DefWindowProcW(window, message, wparam, lparam) }
	}
}
