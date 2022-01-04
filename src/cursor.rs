#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use windows::Win32::Foundation::PWSTR;
use windows::Win32::UI::WindowsAndMessaging::*;

type Type = PWSTR;
pub const AppStarting: Type = IDC_APPSTARTING;
pub const Arrow: Type = IDC_ARROW;
pub const Cross: Type = IDC_CROSS;
pub const Hand: Type = IDC_HAND;
pub const Help: Type = IDC_HELP;
pub const IBeam: Type = IDC_IBEAM;
pub const Icon: Type = IDC_ICON;
pub const No: Type = IDC_NO;
pub const Person: Type = IDC_PERSON;
pub const Pin: Type = IDC_PIN;
pub const Size: Type = IDC_SIZE;
pub const SizeAll: Type = IDC_SIZEALL;
pub const SizeNesw: Type = IDC_SIZENESW;
pub const SizeNs: Type = IDC_SIZENS;
pub const SizeNwse: Type = IDC_SIZENWSE;
pub const SizeWe: Type = IDC_SIZEWE;
pub const UpArrow: Type = IDC_UPARROW;
pub const Wait: Type = IDC_WAIT;
