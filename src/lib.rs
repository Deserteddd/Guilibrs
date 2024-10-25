mod handler;
pub mod gui;
pub mod textfield;
pub mod button;
pub mod slider;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WidgetType {
    Button,
    TextField,
    Slider
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