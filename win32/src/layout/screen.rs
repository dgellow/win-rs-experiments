use std::cmp;

use super::Control;
use crate::{assert::Result, button, input::create_text_input};
use windows::Win32::Foundation::{HINSTANCE, HWND};

#[derive(Default)]
struct Offset {
	left: i32,
	top: i32,
}

#[derive(Default)]
struct Rect {
	w: i32,
	h: i32,
}

pub struct Screen {
	h_instance: HINSTANCE,
	h_window: HWND,
	offset: Offset,
}

impl Screen {
	pub fn new(h_instance: HINSTANCE, h_window: HWND) -> Self {
		Self {
			h_instance,
			h_window,
			offset: Default::default(),
		}
	}

	pub fn render(&mut self, root: Control) -> Result<()> {
		let _rect = self.render_recurse(root)?;
		Ok(())
	}

	fn render_recurse(&mut self, root: Control) -> Result<Rect> {
		match root {
			Control::None => Ok(Default::default()),
			Control::HStack(stack) => {
				let init_x = self.offset.left;
				let init_y = self.offset.top;

				self.offset.left += stack.padding.left;
				self.offset.top += stack.padding.top;

				let mut w = 0;
				let mut h = 0;
				for item in stack.items {
					let rect = self.render_recurse(item)?;
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
					let rect = self.render_recurse(item)?;
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
