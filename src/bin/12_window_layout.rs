use std::cmp;

use gui::{
	assert::Result,
	button, display, err_display,
	input::create_text_input,
	window::{message, Options, WindowBase},
	Point,
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
		use Control::*;

		match message {
			message::Create => {
				display!("WM_CREATE");

				let grid = VStack {
					spacing: 10,
					padding: Padding {
						left: 10,
						..Default::default()
					},
					items: vec![
						HStack {
							spacing: 10,
							padding: Default::default(),
							items: vec![
								InputText {
									text: "hello",
									h: 20,
									w: 100,
									margin: Default::default(),
								},
								InputText {
									text: "world",
									h: 20,
									w: 100,
									margin: Default::default(),
								},
							],
						},
						HStack {
							spacing: 10,
							padding: Default::default(),
							items: vec![
								InputText {
									text: "hello",
									h: 20,
									w: 100,
									margin: Default::default(),
								},
								InputText {
									text: "world",
									h: 20,
									w: 100,
									margin: Default::default(),
								},
							],
						},
						Button {
							title: "My Button 1",
							h: 40,
							w: 100,
							margin: Margin {
								left: 30,
								..Default::default()
							},
						},
					],
				};

				let mut screen = Screen::new(self.h_instance, h_window);
				screen.render(grid).unwrap();

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
			Control::Grid { items } => {
				// self.offset.left += offset.left;
				// self.offset.top += offset.top;
				// for item in items {
				// 	self.render(item)?;
				// }
				todo!()
			}
			Control::HStack {
				items,
				spacing,
				padding,
			} => {
				let init_x = self.offset.left;
				let init_y = self.offset.top;

				self.offset.left += padding.left;
				self.offset.top += padding.top;

				let mut w = 0;
				let mut h = 0;
				for item in items {
					let rect = self.render(item)?;
					// height of hstack should match the largest rendered control width
					h = cmp::max(h, rect.h);
					// width of hstack increased based on size of rendered control + spacing
					w += rect.w + spacing;

					// global horizontal offset should now match initial horizontal position (x) + current vstack width (w)
					self.offset.left = init_x + w;
				}

				// remove last spacing
				w -= spacing;

				// add padding to width and height
				w += padding.left + padding.right;
				h += padding.top + padding.bottom;

				// reset global offset
				self.offset.left = init_x;
				self.offset.top = init_y;

				Ok(Rect { w, h })
			}
			Control::VStack {
				items,
				spacing,
				padding,
			} => {
				let init_x = self.offset.left;
				let init_y = self.offset.top;

				self.offset.left += padding.left;
				self.offset.top += padding.top;

				let mut w = 0;
				let mut h = 0;
				for item in items {
					let rect = self.render(item)?;
					// width of vstack should match the largest rendered control width
					w = cmp::max(w, rect.w);
					// height of vstack increased based on size of rendered control + spacing
					h += rect.h + spacing;

					// global vertical offset should now match initial vertical position (y) + current vstack height (h)
					self.offset.top = init_y + h;
				}

				// remove last spacing
				h -= spacing;

				// add padding to width and height
				w += padding.left + padding.right;
				h += padding.top + padding.bottom;

				// reset global offset
				self.offset.left = init_x;
				self.offset.top = init_y;

				Ok(Rect { w, h })
			}
			Control::Button {
				title,
				w,
				h,
				margin,
			} => {
				button::create(
					self.h_window,
					self.h_instance,
					title,
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
			Control::InputText { text, w, h, margin } => {
				create_text_input(
					self.h_window,
					self.h_instance,
					text,
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

enum Control<'a> {
	VStack {
		padding: Padding,
		items: Vec<Control<'a>>,
		spacing: i32,
	},
	HStack {
		// offset: Offset,
		padding: Padding,
		items: Vec<Control<'a>>,
		spacing: i32,
	},
	Grid {
		// offset: Offset,
		items: Vec<Control<'a>>,
	},
	InputText {
		text: &'a str,
		w: i32,
		h: i32,
		margin: Margin,
	},
	Button {
		title: &'a str,
		w: i32,
		h: i32,
		margin: Margin,
	},
}

#[derive(Default)]
struct Margin {
	right: i32,
	left: i32,
	top: i32,
	bottom: i32,
}

#[derive(Default)]
struct Padding {
	right: i32,
	left: i32,
	top: i32,
	bottom: i32,
}

// enum Container {
// 	Grid(Grid),
// 	VStack(VStack),
// 	HStack(HStack),
// }

// enum Control {
// 	InputText(InputText),
// 	Button(Button),
// }
