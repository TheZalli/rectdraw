use std::ops;
use std::fmt;
use std::cell::Cell;

use termion;

use coord::{Coord, Size};

pub struct GridLayer {
    pos: Coord<u32>,
    size: Size<u32>,
    grid_array: Box<[char]>,
    str_repr: Cell<Option<Box<str>>>,
}

impl GridLayer {
    /// Creates a new empty GridLayer.
    /// 
    /// The GridLayer is filled with the given size of space character (0x20).
    pub fn new(pos: Coord<u32>, size: Size<u32>) -> Self {
        // capacity of grid array
        let cap = size.area() as usize;
        // the grid array
        let array = vec![' '; cap].into_boxed_slice();

        GridLayer {
            pos: pos,
            size: size,
            grid_array: array,
            str_repr: Cell::new(None),
        }
    }

    /// Returns `true` if has a displayable string representation.
    #[inline]
    fn has_str_repr(&self) -> bool {
        unsafe {
            if let None = *self.str_repr.as_ptr() {
                return false;
            } else {
                return true;
            }
        }
    }

    /// Generates the terminal printable string representation for the array.
    fn gen_str_repr(&self) {
        // a shortcut for jumping into the next line
        let jump_next_line = format!("{}{}",
            termion::cursor::Down(1), termion::cursor::Left(
                (self.size.width + 2) as u16
        ));

        // the string to be outputted, start with a command to position the 
        // terminal cursor to the upper-left corner
        let out_str = format!("{}", termion::cursor::Goto(
            (self.pos.x - 1) as u16,
            (self.pos.y - 1) as u16
        ));

        // TODO
    }

    fn get_str_repr<'a>(&'a self) -> &'a str {
        // generate string presentation if needed
        if !self.has_str_repr() {
            self.gen_str_repr();
        }
        unimplemented!()
        /*unsafe {
            self.str_repr.as_ptr().clone().as_ref().unwrap()
        }.unwrap().as_ref()*/
    }


    /// Clears the string representation.
    /// 
    /// Remember to always call when the string representation might become invalid.
    fn clear_str_repr(&mut self) {
        self.str_repr.set(None);
    }
}

impl Clone for GridLayer {
    fn clone(&self) -> Self {
        GridLayer {
            pos: self.pos,
            size: self.size,
            grid_array: self.grid_array.clone(),
            str_repr: Cell::new(None),
        }
    }
}

impl ops::Index<Coord<u32>> for GridLayer {
    type Output = char;
    fn index(&self, index: Coord<u32>) -> &char {
        &self.grid_array[
            index.encode_linear_index(self.size.width as usize)
        ]
    }
}

impl ops::IndexMut<Coord<u32>> for GridLayer {
    fn index_mut(&mut self, index: Coord<u32>) -> &mut char {
        self.clear_str_repr();
        &mut self.grid_array[
            index.encode_linear_index(self.size.width as usize)
        ]
    }
}

impl fmt::Debug for GridLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            write!(f,
                "GridLayer {{ pos: {:?}, size: {:?},
                grid_array: {:?}, str_repr: {:?} }}",
                self.pos, self.size, self.grid_array, *self.str_repr.as_ptr())
        }
    }
}

impl fmt::Display for GridLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.get_str_repr())
    }
}
