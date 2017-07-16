extern crate termion;
extern crate rectdraw;

use std::io;
use std::io::Write;

use termion::raw::IntoRawMode;

use rectdraw::{Size, Coord};
use rectdraw::rectangle::*;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // get the size of the terminal
    let termsize = termion::terminal_size().unwrap_or((80, 24));
    let termsize = Size::new(termsize.0, termsize.1);

    // get the pos and size of the rectangles
    let pos1 = Coord::new(3, 5);
    let pos2 = Coord::new(14, 1);
    let pos3 = Coord::new(14, 7);

    let size = Size::new(4, 4);

    // rectangle characters
    let roguerect = TwoCharRect::new('#', '.');
    let asciirect = FourCharRect::new('+', '-', '|', '.');
    let linerect = UnicodeLineRect::new('.');

    // the rooms
    let rogueroom = rectdraw::AnsiRoom {
        chars: roguerect,
        border_style: String::new(),
        fill_style: String::new(),
        size: size,
    };

    let asciiroom = rectdraw::AnsiRoom {
        chars: asciirect,
        border_style: String::new(),
        fill_style: String::new(),
        size: size,
    };

    let lineroom = rectdraw::AnsiRoom {
        chars: linerect,
        border_style: String::new(),
        fill_style: String::new(),
        size: size,
    };

    // draw

    //write!(stdout, "\x1Bc"); // send reset signal
    write!(stdout, "{}", termion::clear::All).unwrap();

    write!(stdout, "{}", termion::cursor::Goto(pos1.x, pos1.y)).unwrap();
    rogueroom.draw(&mut stdout).unwrap();

    write!(stdout, "{}", termion::cursor::Goto(pos2.x, pos2.y)).unwrap();
    asciiroom.draw(&mut stdout).unwrap();

    write!(stdout, "{}", termion::cursor::Goto(pos3.x, pos3.y)).unwrap();
    lineroom.draw(&mut stdout).unwrap();

    //write!(stdout, "{}", termion::cursor::Goto(termsize.width, termsize.height)).unwrap();
    write!(stdout, "{}", termion::cursor::Down(1)).unwrap();
    stdout.flush().unwrap();
}
