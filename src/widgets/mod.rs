mod button;
mod fader;
mod textfield;

pub use fader::Fader;
use sdl2::rect::Rect;
pub use textfield::TextField;
pub use button::Button;

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

/*
Possible widget trait functions:
    - shift(x, y) -> move widget by x, y
    - bounds -> return widget bounds
    - render -> render widget
    - render_text -> render widget text
*/

pub trait Widget{
    fn shift(&mut self, x: i32, y: i32);
    fn bounds(&self) -> Rect;
    fn visual_bounds(&self) -> Rect {
        self.bounds()
    }
}