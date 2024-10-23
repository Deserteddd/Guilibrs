mod handler;
pub mod gui;
pub mod widget;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WidgetType {
    Button,
    Textbox,
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub trait Widget<T> {
    fn click(&mut self) -> T;
    fn set_label(&mut self, s: &'static str);
}

pub trait RenderText {
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font: &'static str,
    ) -> Result<(), String>;
}
