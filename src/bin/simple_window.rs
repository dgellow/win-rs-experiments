use std::sync::Once;

use gui::{
	display,
	window::{message, show_cmd, style, Class, Window, WindowProc},
};

// static CHILD_CLASS: Option<Class> = None;
const ID_CHILD_1: i32 = 100;
const ID_CHILD_2: i32 = ID_CHILD_1 + 1;
const ID_CHILD_3: i32 = ID_CHILD_1 + 2;

use windows::{
	core::Result,
	Win32::{
		Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
		Graphics::Gdi::ValidateRect,
		System::LibraryLoader::GetModuleHandleW,
		UI::WindowsAndMessaging::*,
	},
};

fn main() -> Result<()> {
	display!("create child class");
	let child_class = Class::new("ChildWindow");

	display!("create main class");
	let main_class = Class::new_with_proc("MainWindow", window_proc);

	display!("create main window");
	let mut main_window =
		main_class.create_window("My Window", Default::default(), Default::default());

	display!("display main window");
	main_window.show();

	display!("update main window");
	main_window.update();

	// start event loop
	main_window.handle_events();

	Ok(())
}

extern "system" fn window_proc(
	window: HWND,
	message: message::Type,
	wparam: WPARAM,
	lparam: LPARAM,
) -> LRESULT {
	let mut rect: Box<RECT> = Box::new(Default::default());

	unsafe {
		match message as message::Type {
			message::Paint => {
				println!("WM_PAINT");
				ValidateRect(window, std::ptr::null());
				0
			}

			// creating main window
			message::Create => {
				println!("WM_CREATE");

				// let child_class = CHILD_CLASS.expect("child class should be registered");
				// get the instance handle
				let h_instance = GetModuleHandleW(None);
				debug_assert_ne!(h_instance, 0, "failed to get module handle");

				for i in 0..3 {
					display!("create child {}", i + 1);
					let h_menu: HMENU = (ID_CHILD_1 + i).try_into().unwrap();
					let child = CreateWindowExW(
						Default::default(),
						"ChildWindow",
						"",
						style::Child | style::Border,
						0,
						0,
						200,
						300,
						None,
						h_menu,
						h_instance,
						std::ptr::null(),
					);

					ShowWindow(child, show_cmd::Show);
				}
				0
			}

			// main window changed size
			message::Size => {
				println!("WM_SIZE");

				GetClientRect(window, rect.as_mut());

				let p: LPARAM = rect.as_mut() as *mut _ as _;
				EnumChildWindows(window, Some(enum_child_proc), p);
				0
			}

			// instance,                                                  // instance handle
			// result.as_mut() as *mut _ as _,                            // window creation data
			message::Destroy => {
				println!("WM_DESTROY");
				PostQuitMessage(0);
				0
			}
			_ => DefWindowProcA(window, message, wparam, lparam),
		}
	}
}

extern "system" fn enum_child_proc(child: HWND, lparam: LPARAM) -> BOOL {
	unsafe {
		display!("enumerate children");

		let id_child = GetWindowLongW(child, GWL_ID);

		let i = match id_child {
			ID_CHILD_1 => 0,
			ID_CHILD_2 => 1,
			_ => 2,
		};

		let parent = lparam as *mut RECT;
		MoveWindow(
			child,
			((*parent).right / 3) * i,
			0,
			(*parent).right / 3,
			(*parent).bottom,
			BOOL(1),
		);

		ShowWindow(child, show_cmd::Show);
	}

	BOOL(1)
}
