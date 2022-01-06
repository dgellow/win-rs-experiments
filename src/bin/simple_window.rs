use gui::window::{message, Window};

use windows::{
	core::Result,
	Win32::{
		Foundation::{HWND, LPARAM, LRESULT, WPARAM},
		Graphics::Gdi::ValidateRect,
		UI::WindowsAndMessaging::{DefWindowProcA, PostQuitMessage},
	},
};

fn main() -> Result<()> {
	let win = Window::new_with_proc(
		"My Window",
		Default::default(),
		Default::default(),
		window_proc,
	)?;

	win.handle_events();
	Ok(())
}

extern "system" fn window_proc(
	window: HWND,
	message: message::Type,
	wparam: WPARAM,
	lparam: LPARAM,
) -> LRESULT {
	unsafe {
		match message as message::Type {
			message::Paint => {
				println!("WM_PAINT");
				ValidateRect(window, std::ptr::null());
				0
			}
			message::Destroy => {
				println!("WM_DESTROY");
				PostQuitMessage(0);
				0
			}
			_ => DefWindowProcA(window, message, wparam, lparam),
		}
	}
}
