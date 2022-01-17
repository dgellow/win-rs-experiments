use derive::{DimensionBuilder, MarginBuilder, PaddingBuilder};

use super::margins::{Dimension, DimensionBuilder, Margin, MarginBuilder, Padding, PaddingBuilder};

pub enum Control {
	None,
	VStack(VStack),
	HStack(HStack),
	InputText(InputText),
	Button(Button),
}

#[derive(Default, PaddingBuilder)]
pub struct VStack {
	pub padding: Padding,
	pub items: Vec<Control>,
	pub spacing: i32,
}

impl VStack {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn spacing(mut self, v: i32) -> Self {
		self.spacing = v;
		self
	}

	pub fn items(mut self, v: Vec<Control>) -> Self {
		self.items = v;
		self
	}

	pub fn done(self) -> Control {
		Control::VStack(self)
	}
}

#[derive(Default, PaddingBuilder)]
pub struct HStack {
	pub padding: Padding,
	pub items: Vec<Control>,
	pub spacing: i32,
}

impl HStack {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn spacing(mut self, v: i32) -> Self {
		self.spacing = v;
		self
	}

	pub fn items(mut self, v: Vec<Control>) -> Self {
		self.items = v;
		self
	}

	pub fn done(self) -> Control {
		Control::HStack(self)
	}
}

#[derive(Default, MarginBuilder, DimensionBuilder)]
pub struct InputText {
	pub text: String,
	pub dimension: Dimension,
	pub margin: Margin,
}

impl InputText {
	pub fn new(text: &str) -> Self {
		Self {
			text: text.to_owned(),
			..Default::default()
		}
	}

	pub fn done(self) -> Control {
		Control::InputText(self)
	}
}

#[derive(Default, MarginBuilder, DimensionBuilder)]
pub struct Button {
	pub title: String,
	pub dimension: Dimension,
	pub margin: Margin,
}

impl Button {
	pub fn new(title: &str) -> Self {
		Self {
			title: title.to_owned(),
			..Default::default()
		}
	}

	pub fn done(self) -> Control {
		Control::Button(self)
	}
}
