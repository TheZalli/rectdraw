use super::{RectPart, RectChars};

pub struct TwoCharRect {
    border: char,
    fill: char,
}

impl TwoCharRect {
    pub fn new(border_char: char, fill_char: char) -> Self {
        TwoCharRect {
            border: border_char,
            fill: fill_char,
        }
    }
}

impl RectChars for TwoCharRect {
    fn get_char_at(&self, part: RectPart) -> char {
        if let RectPart::Center = part {
            self.fill
        } else {
            self.border
        }
    }
}

pub struct FourCharRect {
    corner: char,
    border_horiz: char,
    border_vert: char,
    fill: char,
}


impl FourCharRect {
    pub fn new(corner_char: char,
               horizontal_border_char: char,
               vertical_border_char: char,
               fill_char: char) -> Self
    {
        FourCharRect {
            corner: corner_char,
            border_horiz: horizontal_border_char,
            border_vert: vertical_border_char,
            fill: fill_char,
        }
    }
}

impl RectChars for FourCharRect {
    fn get_char_at(&self, part: RectPart) -> char {
        match part {
            RectPart::Center      => self.fill,
            RectPart::Top         => self.border_horiz,
            RectPart::RightSide   => self.border_vert,
            RectPart::Bottom      => self.border_horiz,
            RectPart::LeftSide    => self.border_vert,
            _                     => self.corner,
        }
    }
}


pub struct UnicodeLineRect {
    fill: char,
}

impl UnicodeLineRect {
    pub fn new(fill_char: char) -> Self {
        UnicodeLineRect {
            fill: fill_char,
        }
    }
}

impl RectChars for UnicodeLineRect {
    fn get_char_at(&self, part: RectPart) -> char {
        match part {
            RectPart::Center      => self.fill,
            RectPart::LeftTop     => '┌',
            RectPart::Top         => '─',
            RectPart::RightTop    => '┐',
            RectPart::RightSide   => '│',
            RectPart::RightBottom => '┘',
            RectPart::Bottom      => '─',
            RectPart::LeftBottom  => '└',
            RectPart::LeftSide    => '│',
        }
    }
}
