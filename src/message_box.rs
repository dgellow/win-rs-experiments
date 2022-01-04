use crate::wide_string::ToWide;
use std::fmt;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn new(
	msg: &str,
	title: &str,
	style: style::Flag,
) -> std::result::Result<Result, MessageBoxError> {
	let w_msg = msg.to_wide();
	let w_title = title.to_wide();
	unsafe {
		let status = MessageBoxW(None, w_msg.as_pwstr(), w_title.as_pwstr(), style);
		match Result::try_from(status) {
			Ok(t) => Ok(t),
			Err(unknown) => Err(MessageBoxError::new(unknown)),
		}
	}
}

#[derive(Debug)]
pub struct MessageBoxError {
	msg: String,
}

impl MessageBoxError {
	fn new(result_value: MESSAGEBOX_RESULT) -> MessageBoxError {
		MessageBoxError {
			msg: format!("unknown message box result value: {}", result_value),
		}
	}
}

impl fmt::Display for MessageBoxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.msg)
	}
}

impl std::error::Error for MessageBoxError {
	fn description(&self) -> &str {
		&self.msg
	}
}

pub enum Result {
	Ok,
	Cancel,
	Abort,
	Retry,
	Ignore,
	Yes,
	No,
	Close,
	Help,
	TryAgain,
	Continue,
	Async,
	Timeout,
}

impl TryFrom<MESSAGEBOX_RESULT> for Result {
	type Error = MESSAGEBOX_RESULT;
	fn try_from(v: MESSAGEBOX_RESULT) -> std::result::Result<Self, Self::Error> {
		match v {
			IDOK => Ok(Result::Ok),
			IDCANCEL => Ok(Result::Cancel),
			IDABORT => Ok(Result::Abort),
			IDRETRY => Ok(Result::Retry),
			IDIGNORE => Ok(Result::Ignore),
			IDYES => Ok(Result::Yes),
			IDNO => Ok(Result::No),
			IDCLOSE => Ok(Result::Close),
			IDHELP => Ok(Result::Help),
			IDTRYAGAIN => Ok(Result::TryAgain),
			IDCONTINUE => Ok(Result::Continue),
			IDASYNC => Ok(Result::Async),
			IDTIMEOUT => Ok(Result::Timeout),
			unknown => Err(unknown),
		}
	}
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod style {
	pub type Flag = MESSAGEBOX_STYLE;

	use windows::Win32::UI::WindowsAndMessaging::*;
	pub const AbortRetryIgnore: Flag = MB_ABORTRETRYIGNORE;
	pub const CancelTryContinue: Flag = MB_CANCELTRYCONTINUE;
	pub const Help: Flag = MB_HELP;
	pub const Ok: Flag = MB_OK;
	pub const OkCancel: Flag = MB_OKCANCEL;
	pub const RetryCancel: Flag = MB_RETRYCANCEL;
	pub const YesNo: Flag = MB_YESNO;
	pub const YesNoCancel: Flag = MB_YESNOCANCEL;
	pub const IconHand: Flag = MB_ICONHAND;
	pub const IconQuestion: Flag = MB_ICONQUESTION;
	pub const IconExclamation: Flag = MB_ICONEXCLAMATION;
	pub const IconAsterisk: Flag = MB_ICONASTERISK;
	pub const UserIcon: Flag = MB_USERICON;
	pub const IconWarning: Flag = MB_ICONWARNING;
	pub const IconError: Flag = MB_ICONERROR;
	pub const IconInformation: Flag = MB_ICONINFORMATION;
	pub const IconStop: Flag = MB_ICONSTOP;
	pub const DefButton1: Flag = MB_DEFBUTTON1;
	pub const DefButton2: Flag = MB_DEFBUTTON2;
	pub const DefButton3: Flag = MB_DEFBUTTON3;
	pub const DefButton4: Flag = MB_DEFBUTTON4;
	pub const ApplModal: Flag = MB_APPLMODAL;
	pub const SystemModal: Flag = MB_SYSTEMMODAL;
	pub const TaskModal: Flag = MB_TASKMODAL;
	pub const NoFocus: Flag = MB_NOFOCUS;
	pub const SetForeground: Flag = MB_SETFOREGROUND;
	pub const DefaultDesktopOnly: Flag = MB_DEFAULT_DESKTOP_ONLY;
	pub const TopMost: Flag = MB_TOPMOST;
	pub const Right: Flag = MB_RIGHT;
	pub const RtlReading: Flag = MB_RTLREADING;
	pub const ServiceNotification: Flag = MB_SERVICE_NOTIFICATION;
	pub const ServiceNotificationNt3x: Flag = MB_SERVICE_NOTIFICATION_NT3X;
	pub const TypeMask: Flag = MB_TYPEMASK;
	pub const IconMask: Flag = MB_ICONMASK;
	pub const DefMask: Flag = MB_DEFMASK;
	pub const ModeMask: Flag = MB_MODEMASK;
	pub const MiscMask: Flag = MB_MISCMASK;
}
