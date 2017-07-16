
fn main() {
	// reset
	print!("\x1Bc");
	// try color and other styles
	println!("\x1B[38;2;241;112;105mHello \x1B[7mWorld!");
	// change title
	print!("\x1B]0;Hello Title!\x1B\\");
}
