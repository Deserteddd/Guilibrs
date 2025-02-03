use crate::rect;
use super::{Orientation, Widget};
use sdl3::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fader {
    position: (i32, i32),
    length: i32,
    orientation: Orientation,
    value: f32,
    range: (f32, f32),
    is_hovered: bool,
    display_on_hover: bool,
}

impl Widget for Fader {
    fn shift(&mut self, x: i32, y: i32) {
        self.position = (self.position.0 + x, self.position.1 + y);
    }
    fn visual_bounds(&self) -> Rect {
        match self.orientation {
            Orientation::Horizontal => rect!(
                self.position.0,
                self.position.1-10,
                self.length,
                20
            ),
            Orientation::Vertical => rect!(
                self.position.0 - 10,
                self.position.1 - self.length,
                20,
                self.length
            )
        }
    }

    fn bounds(&self) -> Rect {
        let lerp = (self.value * self.length as f32) as i32;
        match self.orientation {
            Orientation::Horizontal => rect!(
                self.position.0 + lerp - 5,
                self.position.1 - 10,
                10,
                20
            ),
            Orientation::Vertical => rect!(
                self.position.0 - 10,
                self.position.1 - lerp - 5,
                20,
                10
            )
        }
    }
}

impl Fader {
    pub fn new(x: i32, y: i32, w: i32) -> Fader {
        Fader {
            position: (x, y),
            length: w,
            value: 0.0,
            range: (0.0, 1.0),
            orientation: Orientation::Horizontal,
            is_hovered: false,
            display_on_hover: false,
        }
    }

    pub const fn display_on_hover(mut self) -> Fader {
        self.display_on_hover = true;
        self
    }

    pub const fn vertical(mut self) -> Fader {
        self.orientation = Orientation::Vertical;
        self
    }

    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn initial(mut self, value: f32) -> Fader {
        self.value = (value - self.range.0) / (self.range.1 - self.range.0);
        self
    }

    pub fn value(&self) -> f32 {
        self.range.0 + self.value * (self.range.1 - self.range.0)
    }

    pub const fn range(mut self, min: f32, max: f32) -> Fader {
        self.range = (min, max);
        self
    }

    pub fn increment(&mut self) {
        let value_range = self.range.1 - self.range.0;
        let step = 10.0 / value_range;
        if self.value + step <= 1.0 {
            self.value += step;
        }
    }

    pub fn decrement(&mut self) {
        let value_range = self.range.1 - self.range.0;
        let step = 10.0 / value_range;
        if self.value - step >= 0.0 {
            self.value -= step;
        }
    }

    pub fn drag(&mut self, x: f32, y: f32) {
        match self.orientation {
            Orientation::Horizontal => {
                let x = x - self.position.0 as f32;
                self.value = x / self.length as f32;
                if self.value < 0.0 {
                    self.value = 0.0;
                } else if self.value > 1.0 {
                    self.value = 1.0;
                }
            },
            Orientation::Vertical => {
                let y = self.position.1 as f32 - y;
                self.value = y / self.length as f32;
                if self.value < 0.0 {
                    self.value = 0.0;
                } else if self.value > 1.0 {
                    self.value = 1.0;
                }
            }
        }
    }

    pub fn is_hovered(&mut self, b: bool) {
        self.is_hovered = b;
    }
}