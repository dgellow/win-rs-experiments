use std::cmp;

use gui::{
	assert::Result,
	button, display, err_display,
	input::create_text_input,
	window::{message, Options, WindowBase},
};
use windows::Win32::{
	Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
	Graphics::Gdi::ValidateRect,
};

fn main() -> std::result::Result<(), ()> {
	match app() {
		Ok(_) => Ok(()),
		Err(e) => {
			err_display!("App error: {}", e);
			Err(())
		}
	}
}

fn app() -> Result<()> {
	let main_window = MainWindow::new(
		"MainWindow",
		"Input Window — Win32 💖 Rust",
		Options {
			..Default::default()
		},
	)?;
	display!("main_window: {:?}", main_window);

	let res = MainWindow::event_loop();
	display!("event_loop result: {} ({:#X})", res, res);

	Ok(())
}

#[derive(Debug)]
struct MainWindow {
	h_instance: HINSTANCE,
}

impl WindowBase for MainWindow {
	fn init_state(h_instance: HINSTANCE) -> Self {
		Self { h_instance }
	}

	fn h_instance(&self) -> HINSTANCE {
		self.h_instance
	}

	fn on_message(
		&self,
		h_window: HWND,
		message: message::Type,
		_wparam: WPARAM,
		_lparam: LPARAM,
	) -> Result<gui::window::MessageAction> {
		use gui::window::MessageAction::*;
		// use Control::{HStack, InputText, VStack};

		match message {
			message::Create => {
				display!("WM_CREATE");

				// let button = Button::new("My button 1").leftMargin(30).done();

				let root = VStack {
					spacing: 10,
					padding: Padding {
						left: 10,
						..Default::default()
					},
					items: vec![
						Button::new("My Button 1").leftMargin(30).done(),
						Button {
							title: "My Button 1".to_owned(),
							h: 40,
							w: 100,
							margin: Margin {
								left: 30,
								..Default::default()
							},
						}
						.done(),
						HStack {
							spacing: 10,
							items: vec![
								InputText {
									text: "hello".to_owned(),
									h: 20,
									w: 100,
									margin: Default::default(),
								}
								.done(),
								InputText {
									text: "world".to_owned(),
									h: 20,
									w: 100,
									margin: Default::default(),
								}
								.done(),
							],
							..Default::default()
						}
						.done(),
						HStack {
							spacing: 10,
							padding: Default::default(),
							items: vec![
								InputText {
									text: "hello".to_owned(),
									h: 20,
									w: 100,
									margin: Default::default(),
								}
								.done(),
								InputText {
									text: "world".to_owned(),
									h: 20,
									w: 100,
									margin: Default::default(),
								}
								.done(),
							],
						}
						.done(),
					],
				}
				.done();

				let mut screen = Screen::new(self.h_instance, h_window);
				screen.render(root).unwrap();

				Ok(Continue)
			}
			message::Paint => {
				display!("WM_PAINT");
				unsafe { ValidateRect(h_window, std::ptr::null()) };
				Ok(Continue)
			}
			message::MButtonDown => {
				display!("WM_MBUTTONDOWN");
				Ok(Continue)
			}
			_ => Ok(Continue),
		}
	}
}

#[derive(Default)]
struct Offset {
	left: i32,
	top: i32,
}

struct Screen {
	h_instance: HINSTANCE,
	h_window: HWND,
	offset: Offset,
}

#[derive(Default)]
struct Rect {
	w: i32,
	h: i32,
}

impl Screen {
	fn new(h_instance: HINSTANCE, h_window: HWND) -> Self {
		Self {
			h_instance,
			h_window,
			offset: Default::default(),
		}
	}

	fn render(&mut self, root: Control) -> Result<Rect> {
		match root {
			Control::HStack(stack) => {
				let init_x = self.offset.left;
				let init_y = self.offset.top;

				self.offset.left += stack.padding.left;
				self.offset.top += stack.padding.top;

				let mut w = 0;
				let mut h = 0;
				for item in stack.items {
					let rect = self.render(item)?;
					// height of hstack should match the largest rendered control width
					h = cmp::max(h, rect.h);
					// width of hstack increased based on size of rendered control + spacing
					w += rect.w + stack.spacing;

					// global horizontal offset should now match initial horizontal position (x) + current vstack width (w)
					self.offset.left = init_x + w;
				}

				// remove last spacing
				w -= stack.spacing;

				// add padding to width and height
				w += stack.padding.left + stack.padding.right;
				h += stack.padding.top + stack.padding.bottom;

				// reset global offset
				self.offset.left = init_x;
				self.offset.top = init_y;

				Ok(Rect { w, h })
			}
			Control::VStack(stack) => {
				let init_x = self.offset.left;
				let init_y = self.offset.top;

				self.offset.left += stack.padding.left;
				self.offset.top += stack.padding.top;

				let mut w = 0;
				let mut h = 0;
				for item in stack.items {
					let rect = self.render(item)?;
					// width of vstack should match the largest rendered control width
					w = cmp::max(w, rect.w);
					// height of vstack increased based on size of rendered control + spacing
					h += rect.h + stack.spacing;

					// global vertical offset should now match initial vertical position (y) + current vstack height (h)
					self.offset.top = init_y + h;
				}

				// remove last spacing
				h -= stack.spacing;

				// add padding to width and height
				w += stack.padding.left + stack.padding.right;
				h += stack.padding.top + stack.padding.bottom;

				// reset global offset
				self.offset.left = init_x;
				self.offset.top = init_y;

				Ok(Rect { w, h })
			}
			Control::Button(button) => {
				button::create(
					self.h_window,
					self.h_instance,
					button.title.as_str(),
					self.offset.left + button.margin.left,
					self.offset.top + button.margin.top,
					button.w,
					button.h,
				)?;
				Ok(Rect {
					w: button.w + button.margin.left + button.margin.right,
					h: button.h + button.margin.top + button.margin.bottom,
				})
			}
			Control::InputText(InputText { text, w, h, margin }) => {
				create_text_input(
					self.h_window,
					self.h_instance,
					text.as_str(),
					self.offset.left + margin.left,
					self.offset.top + margin.top,
					w,
					h,
				)?;
				Ok(Rect {
					w: w + margin.left + margin.right,
					h: h + margin.top + margin.bottom,
				})
			}
		}
	}
}

#[derive(Default)]

struct VStack {
	padding: Padding,
	items: Vec<Control>,
	spacing: i32,
}

impl VStack {
	fn done(self) -> Control {
		Control::VStack(self)
	}
}

impl PaddingBuilder for VStack {
	fn get_padding(&self) -> Padding {
		self.padding
	}

	fn set_padding(&mut self, m: Padding) {
		self.padding = m
	}
}

#[derive(Default)]

struct HStack {
	padding: Padding,
	items: Vec<Control>,
	spacing: i32,
}

impl HStack {
	fn done(self) -> Control {
		Control::HStack(self)
	}
}

impl PaddingBuilder for HStack {
	fn get_padding(&self) -> Padding {
		self.padding
	}

	fn set_padding(&mut self, m: Padding) {
		self.padding = m
	}
}

#[derive(Default)]
struct InputText {
	text: String,
	w: i32,
	h: i32,
	margin: Margin,
}

impl InputText {
	fn new(text: &str) -> Self {
		Self {
			text: text.to_owned(),
			..Default::default()
		}
	}

	fn height(mut self, v: i32) -> Self {
		self.h = v;
		self
	}

	fn width(mut self, v: i32) -> Self {
		self.w = v;
		self
	}

	fn done(self) -> Control {
		Control::InputText(self)
	}
}

impl MarginBuilder for InputText {
	fn get_margin(&self) -> Margin {
		self.margin
	}

	fn set_margin(&mut self, m: Margin) {
		self.margin = m
	}
}

enum Control {
	VStack(VStack),
	HStack(HStack),
	InputText(InputText),
	Button(Button),
}

#[derive(Default)]
struct Button {
	title: String,
	w: i32,
	h: i32,
	margin: Margin,
}

impl Button {
	fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
			..Default::default()
		}
	}

	fn height(mut self, v: i32) -> Self {
		self.h = v;
		self
	}

	fn width(mut self, v: i32) -> Self {
		self.w = v;
		self
	}

	fn done(self) -> Control {
		Control::Button(self)
	}
}

impl MarginBuilder for Button {
	fn get_margin(&self) -> Margin {
		self.margin
	}

	fn set_margin(&mut self, m: Margin) {
		self.margin = m
	}
}

#[derive(Default, Clone, Copy)]
struct Margin {
	right: i32,
	left: i32,
	top: i32,
	bottom: i32,
}

trait MarginBuilder
where
	Self: Sized,
{
	fn get_margin(&self) -> Margin;
	fn set_margin(&mut self, m: Margin);

	fn leftMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { left: m, ..mm });
		self
	}

	fn rightMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { right: m, ..mm });
		self
	}

	fn topMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { top: m, ..mm });
		self
	}

	fn bottomMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { bottom: m, ..mm });
		self
	}

	fn verticalMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin {
			top: m,
			bottom: m,
			..mm
		});
		self
	}

	fn horizontalMargin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin {
			left: m,
			right: m,
			..mm
		});
		self
	}
}

#[derive(Default, Clone, Copy)]
struct Padding {
	right: i32,
	left: i32,
	top: i32,
	bottom: i32,
}

trait PaddingBuilder
where
	Self: Sized,
{
	fn get_padding(&self) -> Padding;
	fn set_padding(&mut self, m: Padding);

	fn leftPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { left: m, ..mm });
		self
	}

	fn rightPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { right: m, ..mm });
		self
	}

	fn topPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { top: m, ..mm });
		self
	}

	fn bottomPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { bottom: m, ..mm });
		self
	}

	fn verticalPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding {
			top: m,
			bottom: m,
			..mm
		});
		self
	}

	fn horizontalPadding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding {
			left: m,
			right: m,
			..mm
		});
		self
	}
}
