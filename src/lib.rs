extern crate termion;

pub mod rectangle;
pub mod term_canvas;
mod coord;

use std::io;
use std::iter;

pub use coord::{Coord, Size};
use term_canvas::TermCanvas;

pub enum RectPart {
    Center,
    LeftTop,
    Top,
    RightTop,
    RightSide,
    RightBottom,
    Bottom,
    LeftBottom,
    LeftSide,
}

pub trait RectChars {
    fn get_char_at(&self, part: RectPart) -> char;
}

/// An ansi representation of a rectangular room.
///
/// Size is the size of the floor of the room and pos is the position of the leftmost corner of the
/// wall.
/// Will always be represented by at least 2x2 rectangle (when size is (0,0)).
pub struct AnsiRoom<T: RectChars> {
    pub chars: T,
    pub border_style: String,
    pub fill_style: String,
    pub size: Size<u16>,
}

impl<T: RectChars> AnsiRoom<T> {
    pub fn draw<W: io::Write>(&self, term: &mut W /*, pos: Coord<u16>*/) -> io::Result<()> {
        // a shortcut for jumping into the next line
        let jump_next_line = format!("{}{}",
            termion::cursor::Down(1), termion::cursor::Left(self.size.width + 2)
        );

        // the top line
        let mut top_line: Vec<u8> = format!("{}{}",
            self.border_style, // change style
            self.chars.get_char_at(RectPart::LeftTop), // push the top-left character
        ).into_bytes();

        // the bottom line
        let mut bottom_line: Vec<u8> = format!("{}{}",
            jump_next_line, // jump to next line
            self.chars.get_char_at(RectPart::LeftBottom), // push the left bottom character
        ).into_bytes();

        // the middle line that is repeated
        let mut middle_line: Vec<u8> = Vec::with_capacity(0);

        // if height is larger than 0 init middle_line
        if self.size.height > 0 {
            middle_line = format!("{}{}{}",
                jump_next_line, // jump to next line
                self.chars.get_char_at(RectPart::LeftSide), // push the left side character
                self.fill_style, // change style to fill
            ).into_bytes();
        }


        // if width is larger than 0 get the middle parts of all lines
        // ┌─┐
        // │ │} h >= 0
        // └─┘
        if self.size.width > 0 {
            top_line.append(
                &mut iter::repeat(self.chars.get_char_at(RectPart::Top))
                .take(self.size.width as usize)
                .map(|c| c as u8).collect()
            );

            bottom_line.append(
                &mut iter::repeat(self.chars.get_char_at(RectPart::Bottom))
                .take(self.size.width as usize)
                .map(|c| c as u8).collect()
            );

            // get the middle part of the middle line
            // ┌─┐
            // │ │} h > 0
            // └─┘
            if self.size.height > 0 {
                middle_line.append(
                    &mut iter::repeat(self.chars.get_char_at(RectPart::Center))
                    .take(self.size.width as usize)
                    .map(|c| c as u8).collect()
                );
            }

        }

        // get the right end parts of all lines
        top_line.append(&mut format!("{}",
            self.chars.get_char_at(RectPart::RightTop)
        ).into_bytes());

        bottom_line.append(&mut format!("{}",
            self.chars.get_char_at(RectPart::RightBottom)
        ).into_bytes());

        // get the right end of the middle line
        if self.size.height > 0 {
            middle_line.append(&mut format!("{}{}",
                self.border_style,
                self.chars.get_char_at(RectPart::RightSide)
            ).into_bytes())
        }

        // write all lines
        term.write_all(&top_line)?;
        //write!(term, "{:?}", &top_line);
        for _ in 0..self.size.height {
            term.write_all(&middle_line)?;
        }
        term.write_all(&bottom_line)
    }
}
