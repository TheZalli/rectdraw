use std::ops;
use std::fmt;
use std::cell::Cell;

use termion;

use coord::{Coord, Size};

/// A rectangular matrix of unicode characters.
/// 
/// To read and edit values use the `Index` and `IndexMut` traits.
/// To display on terminal use the `Display` trait.
pub struct TermCanvas {
    pos: Coord<u32>,
    size: Size<u32>,
    grid_array: Box<[char]>,
    str_repr: Cell<Option<String>>,
}

impl TermCanvas {
    /// Creates a new empty TermCanvas.
    /// 
    /// The TermCanvas is filled with the given size of space character (0x20).
    pub fn new(pos: Coord<u32>, size: Size<u32>) -> Self {
        // capacity of grid array
        let cap = size.area() as usize;
        // the grid array
        let array = vec![' '; cap].into_boxed_slice();

        TermCanvas {
            pos: pos,
            size: size,
            grid_array: array,
            str_repr: Cell::new(None),
        }
    }

    pub fn pos(&self) -> Coord<u32> {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: Coord<u32>) {
        self.clear_str_repr();
        self.pos = new_pos;
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    /// Returns `true` if has a displayable string representation.
    #[inline]
    fn has_str_repr(&self) -> bool {
        unsafe {
            (*self.str_repr.as_ptr()).is_some()
        }
    }

    /// Generates the terminal printable string representation for the array.
    fn gen_str_repr(&self) {
        // a shortcut for jumping into the next line
        let jump_next_line = format!("{}{}",
            termion::cursor::Down(1), termion::cursor::Left(
                (self.size.width - 1) as u16
        ));

        // the output, start with a command to position the terminal cursor to 
        // the upper-left corner
        let mut out_str = format!("{}",
            termion::cursor::Goto(
                (self.pos.x + 1) as u16,
                (self.pos.y + 1) as u16
        ));

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                // get the proper linear index
                let idx = Coord::new(x, y)
                    .encode_linear_index(self.size.width as usize);
                // get the pointed character and push it
                out_str.push(self.grid_array[idx]);
            }
            // finished a row
            out_str.push_str(&jump_next_line);
        }

        // store the result
        self.str_repr.set(Some(out_str));
    }

    fn get_str_repr<'a>(&'a self) -> &'a String {
        // generate string presentation if needed
        if !self.has_str_repr() {
            self.gen_str_repr();
        }
        // now when the repr exists return a reference to it
        unsafe {
            &*self.str_repr.as_ptr()
        }.as_ref().unwrap()
    }


    /// Clears the string representation.
    /// 
    /// Remember to always call when the string representation might become invalid.
    #[inline]
    fn clear_str_repr(&mut self) {
        self.str_repr.set(None);
    }
}

impl Clone for TermCanvas {
    fn clone(&self) -> Self {
        TermCanvas {
            pos: self.pos,
            size: self.size,
            grid_array: self.grid_array.clone(),
            str_repr: Cell::new(None),
        }
    }
}

impl ops::Index<Coord<u32>> for TermCanvas {
    type Output = char;
    fn index(&self, index: Coord<u32>) -> &char {
        &self.grid_array[
            index.encode_linear_index(self.size.width as usize)
        ]
    }
}

impl ops::IndexMut<Coord<u32>> for TermCanvas {
    fn index_mut(&mut self, index: Coord<u32>) -> &mut char {
        self.clear_str_repr();
        &mut self.grid_array[
            index.encode_linear_index(self.size.width as usize)
        ]
    }
}

impl fmt::Debug for TermCanvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            write!(f,
                "TermCanvas {{ pos: {:?}, size: {:?},
                grid_array: {:?}, str_repr: {:?} }}",
                self.pos, self.size, self.grid_array, *self.str_repr.as_ptr())
        }
    }
}

impl fmt::Display for TermCanvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO maybe handle out of window bounds cases
        write!(f, "{}", self.get_str_repr())
    }
}
