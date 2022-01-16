use std::cmp;

use derive::{DimensionBuilder, MarginBuilder, PaddingBuilder};
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

				let root = VStack::new(
					10,
					vec![
						HStack::new(
							10,
							vec![
								InputText::new("hello").height(20).width(100).done(),
								InputText::new("world").height(20).width(100).done(),
							],
						)
						.done(),
						HStack::new(
							10,
							vec![
								InputText::new("hello").height(20).width(100).done(),
								InputText::new("world").height(20).width(100).done(),
							],
						)
						.done(),
						Button::new("My Button 1")
							.height(40)
							.width(100)
							.left_margin(30)
							.done(),
					],
				)
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
					button.dimension.width,
					button.dimension.height,
				)?;
				Ok(Rect {
					w: button.dimension.width + button.margin.left + button.margin.right,
					h: button.dimension.height + button.margin.top + button.margin.bottom,
				})
			}
			Control::InputText(input) => {
				create_text_input(
					self.h_window,
					self.h_instance,
					input.text.as_str(),
					self.offset.left + input.margin.left,
					self.offset.top + input.margin.top,
					input.dimension.width,
					input.dimension.height,
				)?;
				Ok(Rect {
					w: input.dimension.width + input.margin.left + input.margin.right,
					h: input.dimension.height + input.margin.top + input.margin.bottom,
				})
			}
		}
	}
}

enum Control {
	VStack(VStack),
	HStack(HStack),
	InputText(InputText),
	Button(Button),
}

#[derive(Default, PaddingBuilder)]
struct VStack {
	padding: Padding,
	items: Vec<Control>,
	spacing: i32,
}

impl VStack {
	fn new(spacing: i32, items: Vec<Control>) -> Self {
		Self {
			spacing,
			items,
			..Default::default()
		}
	}

	fn done(self) -> Control {
		Control::VStack(self)
	}
}

#[derive(Default, PaddingBuilder)]
struct HStack {
	padding: Padding,
	items: Vec<Control>,
	spacing: i32,
}

impl HStack {
	fn new(spacing: i32, items: Vec<Control>) -> Self {
		Self {
			spacing,
			items,
			..Default::default()
		}
	}

	fn done(self) -> Control {
		Control::HStack(self)
	}
}

#[derive(Default, MarginBuilder, DimensionBuilder)]
struct InputText {
	text: String,
	dimension: Dimension,
	margin: Margin,
}

impl InputText {
	fn new(text: &str) -> Self {
		Self {
			text: text.to_owned(),
			..Default::default()
		}
	}

	fn done(self) -> Control {
		Control::InputText(self)
	}
}

#[derive(Default, MarginBuilder, DimensionBuilder)]
struct Button {
	title: String,
	dimension: Dimension,
	margin: Margin,
}

impl Button {
	fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
			..Default::default()
		}
	}

	fn done(self) -> Control {
		Control::Button(self)
	}
}

#[derive(Default)]
struct Dimension {
	height: i32,
	width: i32,
}

trait DimensionBuilder
where
	Self: Sized,
{
	fn set_height(&mut self, v: i32);
	fn set_width(&mut self, v: i32);

	fn height(mut self, v: i32) -> Self {
		self.set_height(v);
		self
	}

	fn width(mut self, v: i32) -> Self {
		self.set_width(v);
		self
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

	fn left_margin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { left: m, ..mm });
		self
	}

	fn right_margin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { right: m, ..mm });
		self
	}

	fn top_margin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { top: m, ..mm });
		self
	}

	fn bottom_margin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin { bottom: m, ..mm });
		self
	}

	fn vertical_margin(mut self, m: i32) -> Self {
		let mm = self.get_margin();
		self.set_margin(Margin {
			top: m,
			bottom: m,
			..mm
		});
		self
	}

	fn horizontal_margin(mut self, m: i32) -> Self {
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

	fn left_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { left: m, ..mm });
		self
	}

	fn right_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { right: m, ..mm });
		self
	}

	fn top_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { top: m, ..mm });
		self
	}

	fn bottom_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding { bottom: m, ..mm });
		self
	}

	fn vertical_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding {
			top: m,
			bottom: m,
			..mm
		});
		self
	}

	fn horizontal_padding(mut self, m: i32) -> Self {
		let mm = self.get_padding();
		self.set_padding(Padding {
			left: m,
			right: m,
			..mm
		});
		self
	}
}
