use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T> {
     pub fn new(x: T, y: T) -> Self {
        Coord { x: x, y: y }
    }
}

impl Coord<u32> {
    /// Encodes an x,y-coordinate into a linear value that could be used as an 
    /// index into a 1D slice.
    ///
    /// Calculates simply `self.y * rows + self.x`.
    /// 
    /// `rows` is the amount of rows in the possible encoded area is.
    /// It has to be greater than zero.
    /// 
    /// If x-coordinate is larger than or equal to `rows`,
    /// the output is is invalid.
    /// 
    /// Assumes zero-based coordinates and gives zero-based output.
    pub fn encode_linear_index(&self, rows: usize) -> usize {
        (self.y as usize) * rows + (self.x as usize)
    }

    /// Decodes a linear index into an x,y-coordinate, reversing
    /// `encode_linear_index`.
    ///
    /// Calculates simply `{ x: linear_index % rows, y: linear_index / rows }`.
    /// 
    /// `rows` is the amount of rows in the possible encoded area is.
    /// It has to be greater than zero and
    /// it has to be the same value as used in encoding.
    pub fn decode_linear_index(linear_index: usize, rows: usize) -> Self {
        // TODO: use a single remainder/division operation from num
        Coord {
            x: (linear_index % rows) as u32,
            y: (linear_index / rows) as u32, // integer division rounds down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width: width, height: height }
    }
}

impl<T: Copy + ops::Mul> Size<T> {
    pub fn area(&self) -> T::Output {
        self.width * self.height
    }
}
