mod button;
mod fader;
mod textfield;

pub use fader::Fader;
pub use textfield::TextField;
pub use button::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetType {
    Button,
    TextField,
    Fader
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left(i32), // i32: padding
    Right(i32), // i32: padding
    Center,
}