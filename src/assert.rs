use windows::Win32::Foundation::GetLastError;

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;

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
					"{}\n\t→ last win32 error: {:#?} ({:#X}). See https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes.",
					e.to_string(),
					win_err,
					win_err
				)
				.into())
			}
		}
	}
}

pub fn assert_eq<Param>(param1: Param, param2: Param, msg: &str) -> Result<()>
where
	Param: std::cmp::PartialEq,
{
	if param1.eq(&param2) {
		return Ok(());
	}
	Err(msg.into())
}

pub fn assert_ne<Param>(param1: Param, param2: Param, msg: &str) -> Result<()>
where
	Param: std::cmp::PartialEq,
{
	if param1.ne(&param2) {
		return Ok(());
	}
	Err(msg.into())
}
