// See https://docs.microsoft.com/en-us/windows/win32/gdi/capturing-an-image#code-example

use derive::WindowBase;
use gui::{
	assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
	defer, display, err_display, loword,
	wide_string::ToWide,
	window::{
		message::{self},
		MessageAction, Options, WindowBase, WindowHandler,
	},
};

use windows::Win32::{
	Foundation::{CloseHandle, HINSTANCE, HWND, LPARAM, RECT, WPARAM},
	Graphics::Gdi::{
		BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteObject, EndPaint,
		GetDC, GetDIBits, GetObjectW, ReleaseDC, SelectObject, SetStretchBltMode, StretchBlt,
		BITMAP, BITMAPFILEHEADER, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HALFTONE,
		HBITMAP, HDC, PAINTSTRUCT, SRCCOPY,
	},
	Storage::FileSystem::{CreateFileW, WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL},
	System::{
		Memory::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GHND},
		SystemServices::GENERIC_WRITE,
	},
	UI::WindowsAndMessaging::{
		DestroyWindow, GetClientRect, GetSystemMetrics, SendMessageW, SM_CXSCREEN, SM_CYSCREEN,
	},
};

fn main() -> std::result::Result<(), ()> {
	let app = App::new("Image Capture — Win32 💖 Rust");

	match app.run() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

#[derive(Default, WindowBase)]
pub struct App {
	h_instance: HINSTANCE,
	h_window: HWND,
	title: String,
}

impl App {
	pub fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
			..Default::default()
		}
	}

	pub fn run(&self) -> Result<()> {
		let _main_window = Self::new_window(
			"MainWindow",
			self.title.as_str(),
			Options {
				..Default::default()
			},
		)?;

		let res = Self::event_loop();
		display!("event_loop result: {} ({:#X})", res, res);

		Ok(())
	}
}

impl WindowHandler for App {
	fn on_create_mut(&mut self) -> Result<MessageAction> {
		// WNDCLASSEXW wcex;

		// wcex.cbSize = sizeof(WNDCLASSEX);

		// wcex.style = CS_HREDRAW | CS_VREDRAW;
		// wcex.lpfnWndProc = WndProc;
		// wcex.cbClsExtra = 0;
		// wcex.cbWndExtra = 0;
		// wcex.hInstance = hInstance;
		// wcex.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_GDICAPTURINGANIMAGE));
		// wcex.hCursor = LoadCursor(nullptr, IDC_ARROW);
		// wcex.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
		// wcex.lpszMenuName = MAKEINTRESOURCEW(IDC_GDICAPTURINGANIMAGE);
		// wcex.lpszClassName = szWindowClass;
		// wcex.hIconSm = LoadIcon(wcex.hInstance, MAKEINTRESOURCE(IDI_SMALL));

		Ok(MessageAction::Continue)
	}

	fn on_message(
		&mut self,
		message: message::Type,
		wparam: WPARAM,
		_lparam: LPARAM,
	) -> Result<MessageAction> {
		use gui::window::MessageAction::*;

		match message {
			message::Command => {
				let control_id: u32 = loword(wparam).try_into().unwrap();
				match control_id {
					app_message::MenuAbout => Ok(FullyHandled),
					app_message::MenuExit => {
						unsafe { DestroyWindow(self.h_window) };
						Ok(FullyHandled)
					}
					_ => Ok(Continue),
				}
			}
			message::Move => {
				unsafe { SendMessageW(self.h_window, message::Paint, 0, 0) };
				Ok(Continue)
			}
			message::Paint => {
				// PAINTSTRUCT ps;
				let mut ps: PAINTSTRUCT = Default::default();
				unsafe {
					let _hdc = BeginPaint(self.h_window, &mut ps as *mut _);
					ScreenCapture::capture(self.h_window).unwrap();
					EndPaint(self.h_window, &ps);
				}

				Ok(FullyHandled)
			}
			_ => Ok(Continue),
		}
	}
}

struct ScreenCapture {
	h_window: HWND,
	hdc_screen: Option<HDC>,
	hdc_window: Option<HDC>,
	hdc_mem_dc: Option<HDC>,
	hbm_screen: Option<HBITMAP>,
}

impl Drop for ScreenCapture {
	fn drop(&mut self) {
		unsafe {
			if let Some(obj) = self.hbm_screen {
				DeleteObject(obj);
				self.hbm_screen = None;
			}
			if let Some(obj) = self.hdc_mem_dc {
				DeleteObject(obj);
				self.hdc_mem_dc = None;
			}
			if let Some(dc) = self.hdc_screen {
				ReleaseDC(0, dc);
				self.hdc_screen = None;
			}
			if let Some(dc) = self.hdc_window {
				ReleaseDC(self.h_window, dc);
				self.hdc_window = None;
			}
		}
	}
}

impl ScreenCapture {
	/// # Safety
	///
	/// Welp
	unsafe fn capture(h_window: HWND) -> Result<()> {
		let mut sc = Self {
			h_window,
			hdc_screen: None,
			hdc_window: None,
			hdc_mem_dc: None,
			hbm_screen: None,
		};

		// Retrieve the handle to a display device context for the client area of the window.
		sc.hdc_screen = {
			let hdc = GetDC(0);
			assert_ne(hdc, 0, "GetDC failed").with_last_win32_err()?;
			Some(hdc)
		};
		sc.hdc_window = {
			let hdc = GetDC(h_window);
			assert_ne(hdc, 0, "GetDC failed").with_last_win32_err()?;
			Some(hdc)
		};

		// Create a compatible DC, which is used in a BitBlt from the window DC.
		sc.hdc_mem_dc = {
			let hdc = CreateCompatibleDC(sc.hdc_window);
			assert_ne(hdc, 0, "CreateCompatibleDC failed").with_last_win32_err()?;
			Some(hdc)
		};

		// Get the client area for size calculation.
		let mut rc_client: RECT = Default::default();
		assert_eq(
			GetClientRect(h_window, &mut rc_client as *mut _).as_bool(),
			true,
			"GetClientRect failed",
		)
		.with_last_win32_err()?;

		// This is the best stretch mode.
		assert_ne(
			SetStretchBltMode(sc.hdc_window, HALFTONE),
			0,
			"SetStretchBltMode failed",
		)
		.with_last_win32_err()?;

		// The source DC is the entire screen, and the destination DC is the current window (HWND).
		assert_eq(
			StretchBlt(
				sc.hdc_window,
				0,
				0,
				rc_client.right,
				rc_client.bottom,
				sc.hdc_screen,
				0,
				0,
				GetSystemMetrics(SM_CXSCREEN),
				GetSystemMetrics(SM_CYSCREEN),
				SRCCOPY,
			)
			.as_bool(),
			true,
			"StretchBlt failed",
		)
		.with_last_win32_err()?;

		// Create a compatible bitmap from the Window DC.
		sc.hbm_screen = {
			let hbm = CreateCompatibleBitmap(
				sc.hdc_window.unwrap(),
				rc_client.right - rc_client.left,
				rc_client.bottom - rc_client.top,
			);
			assert_ne(hbm, 0, "CreateCompatibleBitmap failed").with_last_win32_err()?;
			Some(hbm)
		};

		// Select the compatible bitmap into the compatible memory DC.
		SelectObject(sc.hdc_mem_dc, sc.hbm_screen);

		// Bit block transfer into our compatible memory DC.
		assert_eq(
			BitBlt(
				sc.hdc_mem_dc,
				0,
				0,
				rc_client.right - rc_client.left,
				rc_client.bottom - rc_client.top,
				sc.hdc_window,
				0,
				0,
				SRCCOPY,
			)
			.as_bool(),
			true,
			"BitBlt failed",
		)
		.with_last_win32_err()?;

		// Get the BITMAP from the HBITMAP.
		let mut bmp_screen: BITMAP = Default::default();

		GetObjectW(
			sc.hbm_screen,
			std::mem::size_of::<BITMAP>().try_into().unwrap(),
			&mut bmp_screen as *mut _ as _,
		);

		let mut bmf_header: BITMAPFILEHEADER = Default::default(); // bmfHeader;
		let mut bmi_header = BITMAPINFOHEADER {
			biSize: std::mem::size_of::<BITMAPINFOHEADER>().try_into().unwrap(),
			biWidth: bmp_screen.bmWidth,
			biHeight: bmp_screen.bmHeight,
			biPlanes: 1,
			biBitCount: 32,
			biCompression: BI_RGB.try_into().unwrap(),
			biSizeImage: 0,
			biXPelsPerMeter: 0,
			biYPelsPerMeter: 0,
			biClrUsed: 0,
			biClrImportant: 0,
		};

		let dw_bmp_size: i32 = {
			let bit_count: i32 = bmi_header.biBitCount.try_into().unwrap();
			let mut x = bmp_screen.bmWidth * bit_count + 31;
			x /= 32;
			x *= 4;
			x * bmp_screen.bmHeight
		};

		let h_dib = GlobalAlloc(GHND, dw_bmp_size.try_into().unwrap());
		let lp_bitmap = GlobalLock(h_dib);

		// Gets the "bits" from the bitmap, and copies them into a buffer
		// that's pointed to by lpbitmap.
		// let bmi: *mut BITMAPINFO = &mut bmi_header as *mut BITMAPINFOHEADER as _;
		GetDIBits(
			sc.hdc_window,
			sc.hbm_screen,
			0,
			bmp_screen.bmHeight.try_into().unwrap(),
			lp_bitmap,
			&mut bmi_header as *mut _ as _,
			DIB_RGB_COLORS,
		);

		// A file is created, this is where we will save the screen capture.
		let h_file = CreateFileW(
			"captureqwsx.bmp".to_wide().as_pwstr(),
			GENERIC_WRITE,
			0,
			std::ptr::null(),
			CREATE_ALWAYS,
			FILE_ATTRIBUTE_NORMAL,
			None,
		);

		let size_of_bmf: u32 = std::mem::size_of::<BITMAPFILEHEADER>().try_into().unwrap();
		let size_of_bmi: u32 = std::mem::size_of::<BITMAPINFOHEADER>().try_into().unwrap();

		// Add the size of the headers to the size of the bitmap to get the total file size.
		let dw_size_of_dib: u32 = {
			let bmp_size: u32 = dw_bmp_size.try_into().unwrap();
			bmp_size + size_of_bmf + size_of_bmi
		};

		// Offset to where the actual bitmap bits start.
		bmf_header.bfOffBits = size_of_bmf + size_of_bmi;

		// Size of the file.
		bmf_header.bfSize = dw_size_of_dib;

		// bfType must always be BM for Bitmaps.
		bmf_header.bfType = 0x4D42; // BM.

		let mut bytes_written: u32 = 0;
		WriteFile(
			h_file,
			&bmf_header as *const _ as _,
			size_of_bmf,
			&mut bytes_written as *mut _,
			std::ptr::null_mut(),
		);
		WriteFile(
			h_file,
			&bmi_header as *const _ as _,
			size_of_bmi,
			&mut bytes_written as *mut _,
			std::ptr::null_mut(),
		);
		WriteFile(
			h_file,
			lp_bitmap,
			dw_bmp_size.try_into().unwrap(),
			&mut bytes_written as *mut _,
			std::ptr::null_mut(),
		);

		// Unlock and Free the DIB from the heap.
		// TODO: should be handled in drop() too
		GlobalUnlock(h_dib);
		GlobalFree(h_dib);

		// Close the handle for the file that was created.
		// TODO: should be handled in drop() too
		CloseHandle(h_file);

		Ok(())
	}
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod app_message {
	use windows::Win32::UI::WindowsAndMessaging::WM_USER;
	pub type Type = u32;

	pub const MenuAbout: Type = WM_USER;
	pub const MenuExit: Type = WM_USER + 1;
}
