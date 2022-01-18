use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};

use crate::{
	assert::{assert_eq, Result},
	wide_string::ToWide,
};

pub fn is_light_theme() -> Result<bool> {
	// based on https://stackoverflow.com/questions/51334674/how-to-detect-windows-10-light-dark-mode-in-win32-application
	let mut buffer: [u8; 4] = [0; 4];
	let mut cb_data: u32 = (buffer.len()).try_into().unwrap();
	let res = unsafe {
		RegGetValueW(
			HKEY_CURRENT_USER,
			r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#
				.to_wide()
				.as_pwstr(),
			"AppsUseLightTheme".to_wide().as_pwstr(),
			RRF_RT_REG_DWORD,
			std::ptr::null_mut(),
			buffer.as_mut_ptr() as _,
			&mut cb_data as *mut _,
		)
	};
	assert_eq(
		res,
		0,
		format!("failed to read key from registry: err_code={}", res).as_str(),
	)?;

	let light_mode = i32::from_le_bytes(buffer) == 1;
	Ok(light_mode)
}

pub fn is_dark_theme() -> Result<bool> {
	Ok(!is_light_theme()?)
}
