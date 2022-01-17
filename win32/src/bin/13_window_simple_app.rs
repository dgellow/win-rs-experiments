use gui::{
	assert::Result,
	layout::{Button, DimensionBuilder, HStack, InputText, MarginBuilder, PaddingBuilder, VStack},
	SimpleApp,
};

fn main() -> Result<()> {
	let app = SimpleApp::new("Simple App Window — Win32 💖 Rust", || {
		VStack::new()
			.left_padding(10)
			.spacing(10)
			.items(vec![
				HStack::new()
					.spacing(10)
					.items(vec![
						InputText::new("hello").height(20).width(100).done(),
						InputText::new("world").height(20).width(100).done(),
					])
					.done(),
				HStack::new()
					.spacing(10)
					.items(vec![
						InputText::new("hello").height(20).width(100).done(),
						InputText::new("world").height(20).width(100).done(),
					])
					.done(),
				Button::new("My Button 1")
					.height(40)
					.width(100)
					.left_margin(30)
					.done(),
			])
			.done()
	});
	app.run()
}
