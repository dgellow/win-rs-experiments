use std::fmt::Debug;

use windows::Win32::Foundation::{GetLastError, WIN32_ERROR};

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;

fn win32_error_codes_url(win_err: WIN32_ERROR) -> String {
	let base_url: String =
		"https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes".to_owned();
	let url = match win_err {
		0..=499 => base_url + "--0-499-",
		500..=999 => base_url + "--500-999-",
		1000..=1299 => base_url + "--1000-1299-",
		1300..=1699 => base_url + "--1300-1699-",
		1700..=3999 => base_url + "--1700-3999-",
		4000..=5999 => base_url + "--4000-5999-",
		6000..=8199 => base_url + "--6000-8199-",
		8200..=8999 => base_url + "--8200-8999-",
		9000..=11999 => base_url + "--9000-11999-",
		12000..=15999 => base_url + "--12000-15999-",
		_ => base_url,
	};
	format!("{}?code={}", url, win_err)
}

pub trait WithLastWin32Error<T> {
	fn with_last_win32_err(self) -> Result<T>;
}

impl<T> WithLastWin32Error<T> for Result<T> {
	fn with_last_win32_err(self) -> Result<T> {
		match self {
			Ok(i) => Ok(i),
			Err(e) => {
				let win_err = unsafe { GetLastError() };
				Err(format!(
					"{}\n\t→ Last win32 error: {:#?} ({:#X}).\n\t  See {}.",
					e.to_string(),
					win_err,
					win_err,
					win32_error_codes_url(win_err),
				)
				.into())
			}
		}
	}
}

pub trait WrappedError<T> {
	fn wrap_err(self, msg: &str) -> Result<T>;
}

impl<T> WrappedError<T> for Result<T> {
	fn wrap_err(self, msg: &str) -> Result<T> {
		match self {
			Ok(i) => Ok(i),
			Err(e) => Err(format!("{}: {}", msg, e.to_string(),).into()),
		}
	}
}

pub fn assert_eq<Param>(param1: Param, param2: Param, msg: &str) -> Result<()>
where
	Param: std::cmp::PartialEq + Debug,
{
	if param1.eq(&param2) {
		return Ok(());
	}
	Err(format!("{} (param1={:?}, param2={:?})", msg, param1, param2).into())
}

pub fn assert_ne<Param>(param1: Param, param2: Param, msg: &str) -> Result<()>
where
	Param: std::cmp::PartialEq + Debug,
{
	if param1.ne(&param2) {
		return Ok(());
	}
	Err(format!("{} (param1={:?}, param2={:?})", msg, param1, param2).into())
}

pub fn assert_not_null<Param>(param: *const Param, msg: &str) -> Result<()> {
	if param.is_null() {
		return Err(msg.into());
	}
	Ok(())
}
