mod button;
mod fader;
mod textfield;
mod dropdownbutton;

pub use fader::Fader;
pub use textfield::TextField;
pub use button::Button;
pub use dropdownbutton::DropdownButton;

pub type WidgetData = (&'static str, WidgetType, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetType {
    Button,
    TextField,
    Fader,
    DropdownButton
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
    fn bounds(&self) -> sdl3::rect::Rect;
    fn visual_bounds(&self) -> sdl3::rect::Rect {
        self.bounds()
    }
}