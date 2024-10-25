use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;

use crate::widgets::Button;
use crate::widgets::Fader;
use crate::widgets::TextField;
use crate::{Render, RenderText};

#[derive(Debug, Clone, PartialEq)]
pub struct Panel<T> 
where
    T: Copy,
{
    pub buttons: Vec<Button<T>>,
    pub textfields: Vec<TextField>,
    pub faders: Vec<Fader>,
    pub font: &'static str
}

impl<T> Panel<T> 
where 
    T: Copy,
{
    pub const fn new(
        buttons: Vec<Button<T>>,
        textfields: Vec<TextField>,
        faders: Vec<Fader>
    ) -> Panel<T> {
        Panel { buttons, textfields, faders, font: crate::DEFAULTFONT }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext) -> Result<(), String> {
        for button in &self.buttons {
            button.render(canvas)?;
            button.render_text(ttf, canvas, self.font)?;
        }
        for textfield in &self.textfields {
            textfield.render(canvas)?;
            textfield.render_text(ttf, canvas, self.font)?;
        }
        for fader in &self.faders {
            fader.render(canvas)?;
            fader.render_text(ttf, canvas, self.font)?;
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty() && self.textfields.is_empty() && self.faders.is_empty()
    }
}

