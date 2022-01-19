use windows::Win32::{
	Foundation::{GetLastError, SetLastError, HWND},
	UI::WindowsAndMessaging::{
		GetWindowLongPtrW, GetWindowLongW, SetWindowLongPtrW, SetWindowLongW, WINDOW_LONG_PTR_INDEX,
	},
};

use crate::assert::{assert_eq, Result, WithLastWin32Error};

pub fn get_window_long_ptr(h_window: HWND, index: WINDOW_LONG_PTR_INDEX) -> Result<isize> {
	// About GetWindowLongPtr return value.
	//
	// Quote:
	//   If the function succeeds, the return value is the requested value.
	//
	//   If the function fails, the return value is zero. To get extended error information, call GetLastError.
	//
	//   If SetWindowLong or SetWindowLongPtr has not been called previously, GetWindowLongPtr returns zero for values
	//   in the extra window or class memory.
	//
	// Source: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#return-value
	unsafe {
		SetLastError(0);
		let ret = GetWindowLongPtrW(h_window, index);
		let last_err = GetLastError();
		assert_eq(last_err, 0, "GetWindowLongPtrW error").with_last_win32_err()?;
		Ok(ret)
	}
}

pub fn set_window_long_ptr(
	h_window: HWND,
	index: WINDOW_LONG_PTR_INDEX,
	val: isize,
) -> Result<isize> {
	// About SetWindowLongPtr return value.
	//
	// Quote:
	//   If the function succeeds, the return value is the previous value of the specified offset.
	//
	//   If the function fails, the return value is zero. To get extended error information, call GetLastError.
	//
	//   If the previous value is zero and the function succeeds, the return value is zero, but the function does not
	//   clear the last error information. To determine success or failure, clear the last error information by calling
	//   SetLastError with 0, then call SetWindowLongPtr. Function failure will be indicated by a return value of zero
	//   and a GetLastError result that is nonzero.
	//
	// Source: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw#return-value
	unsafe {
		SetLastError(0);
		let ret = SetWindowLongPtrW(h_window, index, val);
		let last_err = GetLastError();
		assert_eq(last_err, 0, "SetWindowLongPtrW error").with_last_win32_err()?;
		Ok(ret)
	}
}

pub fn get_window_long(h_window: HWND, index: WINDOW_LONG_PTR_INDEX) -> Result<i32> {
	// About GetWindowLong return value.
	//
	// Quote:
	//   If the function succeeds, the return value is the requested value.
	//
	//   If the function fails, the return value is zero. To get extended error information, call GetLastError.
	//
	//   If SetWindowLong has not been called previously, GetWindowLong returns zero for values in the extra window or
	//   class memory.
	//
	// Source: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlong#return-value
	unsafe {
		SetLastError(0);
		let ret = GetWindowLongW(h_window, index);
		let last_err = GetLastError();
		assert_eq(last_err, 0, "GetWindowLongW error").with_last_win32_err()?;
		Ok(ret)
	}
}

pub fn set_window_long(h_window: HWND, index: WINDOW_LONG_PTR_INDEX, val: i32) -> Result<i32> {
	// About SetWindowLong return value.
	//
	// Quote:
	//   If the function succeeds, the return value is the previous value of the specified 32-bit integer.
	//
	//   If the function fails, the return value is zero. To get extended error information, call GetLastError.
	//
	//   If the previous value of the specified 32-bit integer is zero, and the function succeeds, the return value is
	//   zero, but the function does not clear the last error information. This makes it difficult to determine success
	//   or failure. To deal with this, you should clear the last error information by calling SetLastError with 0
	//   before calling SetWindowLong. Then, function failure will be indicated by a return value of zero and a
	//   GetLastError result that is nonzero.
	//
	// Source: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongw#return-value
	unsafe {
		SetLastError(0);
		let ret = SetWindowLongW(h_window, index, val);
		let last_err = GetLastError();
		assert_eq(last_err, 0, "SetWindowLongW error").with_last_win32_err()?;
		Ok(ret)
	}
}
