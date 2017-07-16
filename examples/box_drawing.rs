use std::char;

const OFFSET: u32 = 0x2500;

fn main() {
	// draw the unicode box drawing characters
	println!("{}Unicode box drawing characters{}", "\x1B[1m", "\x1B[0m");
	print_box_chars();

	println!("\n{}Unicode box drawing characters categorized{}", "\x1B[1m", "\x1B[0m");
	println!("");
	print_chars_sorted();
}

fn print_box_chars() {
	// draw the top grid
	print!("     ");
	for row in 0x0..0xF+1 {
		print!("{:X} ", row);
	}
	println!("\n");

	for col in 0x0..0x7+1 {
		// left grid label
		print!("{:X} ", OFFSET + col*0x10);
		for row in 0x0..0xF+1 {
			// draw  the row's characters
			print!("{} ", char::from_u32(col*0x10 + row + OFFSET).unwrap());
		}
		println!("\n");
	}
}

fn print_chars_sorted() {
	println!("light:");
	println!(" ─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼\n");

	println!("heavy:");
	println!(" ━ ┃ ┏ ┓ ┗ ┛ ┣ ┫ ┳ ┻ ╋\n");

	println!("double:");
	println!(" ═ ║ ╔ ╗ ╚ ╝ ╠ ╣ ╦ ╩ ╬\n");

	println!("light+heavy:");
	println!(" ╼ ╽ ╾ ╿ ┍ ┎ ┑ ┒ ┕ ┖ ┙ ┚\n");
	println!(" ┝ ┞ ┟ ┠ ┡ ┢ ┥ ┦ ┧ ┨ ┩ ┪ ┭ ┮ ┯ ┰ ┱ ┲ ┵ ┶ ┷ ┸ ┹ ┺\n", );
	println!(" ┽ ┾ ┿ ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊\n");

	println!("light+double:");
	println!(" ╒ ╓ ╕ ╖ ╘ ╙ ╛ ╜\n");
	println!(" ╞ ╟ ╡ ╢ ╪ ╫\n");
}

/*

styles with all characters:
- light: 11
	- straight lines: 2
	- corners: 4
	- t-crosses: 4
	- crosses: 1
- heavy: 11
- double: 11
- light + heavy: 50
	- straight lines: 4
	- corners: 8
	- t-crosses: 24
	- crosses: 14
- light + double: 14
	- straiht lines: 0
	- corners: 8
	- t-crosses: 4
	- crosses: 2

styles with only straight lines:
- light double dash: 2
- light triple dashed: 2
- light quadruple dashed: 2
- heavy double dashed: 2
- heavy triple dashed: 2
- heavy quadruple dashed: 2
- light half line: 4
- thick half line: 4

styles with only corners:
- arc: 4

*/
