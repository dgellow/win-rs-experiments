use crate::{
	assert::{assert_eq, assert_ne, Result, WithLastWin32Error},
	wide_string::ToWide,
};
use windows::Win32::{
	Foundation::{GetLastError, SetLastError, HANDLE, HWND},
	UI::WindowsAndMessaging::{
		GetPropW, GetWindowLongPtrW, GetWindowLongW, RemovePropW, SetPropW, SetWindowLongPtrW,
		SetWindowLongW, WINDOW_LONG_PTR_INDEX,
	},
};

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

pub fn set_property<T>(window: HWND, name: &str, val: &mut T) -> Result<()> {
	// About SetProp return value.
	//
	// Quote:
	//   If the data handle and string are added to the property list, the return value is nonzero.
	//   If the function fails, the return value is zero. To get extended error information, call GetLastError.
	//
	// Source: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setpropw#return-value
	let ptr: isize = val as *mut _ as _;
	let ret = unsafe { SetPropW(window, name.to_wide().as_pwstr(), HANDLE(ptr)) };
	assert_ne(ret.as_bool(), false, "SetPropW error").with_last_win32_err()?;
	Ok(())
}

pub fn get_property<T>(window: HWND, name: &str) -> Result<*mut T> {
	let ret = unsafe { GetPropW(window, name.to_wide().as_pwstr()) };
	let handle = match ret.ok() {
		Ok(h) => h.0,
		Err(e) => {
			return Err(format!("invalid handle returned by GetPropW: {:?}", e.message()).into())
				.with_last_win32_err();
		}
	};
	let val: *mut T = handle as *mut _;
	Ok(val)
}

pub fn remove_property<T>(window: HWND, name: &str) -> Result<*mut T> {
	// About RemoveProp return value.
	//
	// Quote:
	//   The return value identifies the specified data. If the data cannot be found in the specified property list, the
	//   return value is NULL.
	//
	//   The return value is the hData value that was passed to SetProp; it is an application-defined value. Note, this
	//   function only destroys the association between the data and the window. If appropriate, the application must
	//   free the data handles associated with entries removed from a property list. The application can remove only
	//   those properties it has added. It must not remove properties added by other applications or by the system
	//   itself.
	//
	//   The RemoveProp function returns the data handle associated with the string so that the application can free the
	//   data associated with the handle.
	//
	//   Starting with Windows Vista, RemoveProp is subject to the restrictions of User Interface Privilege Isolation
	//   (UIPI). A process can only call this function on a window belonging to a process of lesser or equal integrity
	//   level. When UIPI blocks property changes, GetLastError will return 5.
	//
	// Sources:
	// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removepropa#return-value
	// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removepropa#remarks
	let ret = unsafe { RemovePropW(window, name.to_wide().as_pwstr()) };
	let handle = match ret.ok() {
		Ok(h) => h.0,
		Err(e) => {
			return Err(
				format!("invalid handle returned by RemovePropW: {:?}", e.message()).into(),
			)
			.with_last_win32_err();
		}
	};
	let val: *mut T = handle as *mut _;
	Ok(val)
}
