#[derive(Default)]
pub struct Dimension {
	pub height: i32,
	pub width: i32,
}

pub trait DimensionBuilder
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
pub struct Margin {
	pub right: i32,
	pub left: i32,
	pub top: i32,
	pub bottom: i32,
}

pub trait MarginBuilder
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
pub struct Padding {
	pub right: i32,
	pub left: i32,
	pub top: i32,
	pub bottom: i32,
}

pub trait PaddingBuilder
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
