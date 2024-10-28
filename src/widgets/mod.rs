mod button;
mod fader;
mod textfield;

pub use fader::Fader;
pub use textfield::TextField;
pub use button::Button;

pub type WidgetData = (&'static str, WidgetType, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetType {
    Button,
    TextField,
    Fader
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left(i32), // i32: padding
    Right(i32), // i32: padding
    Center,
}


pub trait Widget{
    fn shift(&mut self, x: i32, y: i32);
    fn bounds(&self) -> sdl2::rect::Rect;
    fn visual_bounds(&self) -> sdl2::rect::Rect {
        self.bounds()
    }
}