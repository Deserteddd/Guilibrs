mod handler;
mod gui;
mod panel;
pub mod widgets;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub use crate::gui::GUI;
pub use crate::panel::Panel;

const DEFAULTFONT: &'static str = "./Courier_Prime.ttf";
const BACKROUNDCOLOR: Color = Color::RGB(40, 40, 40);



pub enum GuiEvent<T> {
    Quit,
    Callback(T),
    FaderUpdate(usize, f32),
    None
}


pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub trait RenderText {
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font: &'static str,
    ) -> Result<(), String>;
}