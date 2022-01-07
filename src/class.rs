use windows::Win32::{
	Foundation::{HWND, LPARAM, LRESULT, WPARAM},
	System::LibraryLoader::GetModuleHandleW,
	UI::WindowsAndMessaging::{LoadCursorW, RegisterClassExW, WNDCLASSEXW},
};

use crate::{wide_string::ToWide, window::message};
