#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use windows::Win32::{Foundation::PWSTR, UI::WindowsAndMessaging::*};

pub const Application: PWSTR = IDI_APPLICATION;
pub const Asterisk: PWSTR = IDI_ASTERISK;
pub const Error: u32 = IDI_ERROR;
pub const Exclamation: PWSTR = IDI_EXCLAMATION;
pub const Hand: PWSTR = IDI_HAND;
pub const Information: u32 = IDI_INFORMATION;
pub const Question: PWSTR = IDI_QUESTION;
pub const Shield: PWSTR = IDI_SHIELD;
pub const Warning: u32 = IDI_WARNING;
pub const WinLogo: PWSTR = IDI_WINLOGO;
