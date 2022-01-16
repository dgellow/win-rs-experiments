use windows::Win32::UI::Controls::*;

pub fn new(
	msg: &str,
	title: &str,
	style: style::Flag,
) -> std::result::Result<Result, MessageBoxError> {
	unsafe {
		let status = TaskDialog(
			None,
			None,
			"My Title",
			"My Main Instructions",
			"My Content",
			button_flags::Yes | button_flags::No,
			0,
		);
		match Result::try_from(status) {
			Ok(t) => Ok(t),
			Err(unknown) => Err(MessageBoxError::new(unknown)),
		}
	}
}

pub mod button_flags {
	pub type Type = TASKDIALOG_COMMON_BUTTON_FLAGS;
	pub const Ok: Type = TDCBF_OK_BUTTON;
	pub const Yes: Type = TDCBF_YES_BUTTON;
	pub const No: Type = TDCBF_NO_BUTTON;
	pub const Cancel: Type = TDCBF_CANCEL_BUTTON;
	pub const Retry: Type = TDCBF_RETRY_BUTTON;
	pub const Close: Type = TDCBF_CLOSE_BUTTON;
}

pub mod elements {
	pub type Type = TASKDIALOG_ELEMENTS;
	pub const Content: Type = TDE_CONTENT;
	pub const ExpandedInformation: Type = TDE_EXPANDED_INFORMATION;
	pub const Footer: Type = TDE_FOOTER;
	pub const MainInstruction: Type = TDE_MAIN_INSTRUCTION;
}

pub mod flags {
	pub type Type = TASKDIALOG_FLAGS;
	pub const EnableHyperlinks: Type = TDF_ENABLE_HYPERLINKS;
	pub const UseHIconMain: Type = TDF_USE_HICON_MAIN;
	pub const UseHIconFooter: Type = TDF_USE_HICON_FOOTER;
	pub const AllowDialogCancellation: Type = TDF_ALLOW_DIALOG_CANCELLATION;
	pub const CommandLinks: Type = TDF_USE_COMMAND_LINKS;
	pub const CommandLinksNoIcon: Type = TDF_USE_COMMAND_LINKS_NO_ICON;
	pub const ExpandFooterArea: Type = TDF_EXPAND_FOOTER_AREA;
	pub const ExpandedByDefault: Type = TDF_EXPANDED_BY_DEFAULT;
	pub const VerificationFlagChecked: Type = TDF_VERIFICATION_FLAG_CHECKED;
	pub const ShowProgressBar: Type = TDF_SHOW_PROGRESS_BAR;
	pub const ShowMarqueeProgressBar: Type = TDF_SHOW_MARQUEE_PROGRESS_BAR;
	pub const CallbackTimer: Type = TDF_CALLBACK_TIMER;
	pub const PositionRelativeToWindow: Type = TDF_POSITION_RELATIVE_TO_WINDOW;
	pub const RtlLayout: Type = TDF_RTL_LAYOUT;
	pub const NoDefaultRadioButton: Type = TDF_NO_DEFAULT_RADIO_BUTTON;
	pub const CanBeMinimized: Type = TDF_CAN_BE_MINIMIZED;
	pub const NoSetForeground: Type = TDF_NO_SET_FOREGROUND;
	pub const SizeToContent: Type = TDF_SIZE_TO_CONTENT;
}

pub mod icon_elements {
	pub type Type = TASKDIALOG_ICON_ELEMENTS;
	pub const IconMain: Type = 0i32;
	pub const IconFooter: Type = 1i32;
}

pub mod messages {
	pub type Type = TASKDIALOG_MESSAGES;
	pub const NavigatePage: Type = TDM_NAVIGATE_PAGE;
	pub const ClickButton: Type = TDM_CLICK_BUTTON;
	pub const SetMarqueeProgressBar: Type = TDM_SET_MARQUEE_PROGRESS_BAR;
	pub const SetPRogressBarState: Type = TDM_SET_PROGRESS_BAR_STATE;
	pub const SetProgressBarRange: Type = TDM_SET_PROGRESS_BAR_RANGE;
	pub const SetProgressBarPos: Type = TDM_SET_PROGRESS_BAR_POS;
	pub const SetProgressBarMarquee: Type = TDM_SET_PROGRESS_BAR_MARQUEE;
	pub const SetElementText: Type = TDM_SET_ELEMENT_TEXT;
	pub const ClickRadioButton: Type = TDM_CLICK_RADIO_BUTTON;
	pub const EnableButton: Type = TDM_ENABLE_BUTTON;
	pub const EnableRadioButton: Type = TDM_ENABLE_RADIO_BUTTON;
	pub const ClickVerification: Type = TDM_CLICK_VERIFICATION;
	pub const UpdateElementText: Type = TDM_UPDATE_ELEMENT_TEXT;
	pub const SetButtonElevationRequiredState: Type = TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE;
	pub const UpdateIcon: Type = TDM_UPDATE_ICON;
}

pub mod notifications {
	pub type Type = TASKDIALOG_NOTIFICATIONS;
	pub const Created: Type = TDN_CREATED;
	pub const Navigated: Type = TDN_NAVIGATED;
	pub const ButtonClicked: Type = TDN_BUTTON_CLICKED;
	pub const HyperlinkClicked: Type = TDN_HYPERLINK_CLICKED;
	pub const Timer: Type = TDN_TIMER;
	pub const Destroyed: Type = TDN_DESTROYED;
	pub const RadioButtonClicked: Type = TDN_RADIO_BUTTON_CLICKED;
	pub const DialogConstructed: Type = TDN_DIALOG_CONSTRUCTED;
	pub const VerificationClicked: Type = TDN_VERIFICATION_CLICKED;
	pub const Help: Type = TDN_HELP;
	pub const ExpandoButtonClicked: Type = TDN_EXPANDO_BUTTON_CLICKED;
}
