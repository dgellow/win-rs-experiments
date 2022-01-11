pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
	let r32: u32 = r.into();
	let g32: u32 = g.into();
	let b32: u32 = b.into();

	r32 | (g32 << 8) | (b32 << 16)
}
