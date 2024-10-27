use crate::{Render, RenderText, DEBUG};
use super::Widget;
use sdl2::{pixels::Color, rect::Rect};
use sdl2::render::TextureQuery;

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fader {
    position: (i32, i32),
    width: i32,
    value: f32,
    range: (f32, f32),
    is_hovered: bool,
    display_on_hover: bool,
}

impl Widget for Fader {
    fn shift(&mut self, x: i32, y: i32) {
        self.position = (self.position.0 + x, self.position.1 + y);
    }
    fn bounds(&self) -> Rect {
        rect!(self.position.0, self.position.1 - 10, self.width, 20)
    }
}

impl Fader {
    pub fn new(x: i32, y: i32, w: i32) -> Fader {
        Fader {
            position: (x, y),
            width: w,
            value: 0.0,
            range: (0.0, 1.0),
            is_hovered: false,
            display_on_hover: false,
        }
    }

    pub const fn display_on_hover(mut self) -> Fader {
        self.display_on_hover = true;
        self
    }



    pub fn rect(&self) -> Rect {
        let lerp = (self.value * self.width as f32) as i32;
        Rect::new(
                self.position.0 + lerp - 5,
                self.position.1 - 10,
                10,
                20
        )
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

    pub fn drag(&mut self, x: i32) {
        let x = x - self.position.0;
        self.value = x as f32 / self.width as f32;
        if self.value < 0.0 {
            self.value = 0.0;
        } else if self.value > 1.0 {
            self.value = 1.0;
        }
    }

    pub fn is_hovered(&mut self, b: bool) {
        self.is_hovered = b;
    }
}

impl Render for Fader {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let lerp = (self.value * self.width as f32) as i32;
        for i in 0..3 {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
            canvas.draw_line(
                (self.position.0, self.position.1 + i),
                (self.position.0 + self.width, self.position.1 + i)
            )?;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
            canvas.draw_line(
                (self.position.0, self.position.1 + i),
                (self.position.0 + lerp, self.position.1 + i)
            )?;
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
        canvas.fill_rect(Rect::new(
            self.position.0 + lerp - 5,
            self.position.1 - 10,
            10,
            20
        ))?;
        if self.is_hovered {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
        }        
        canvas.draw_rect(self.rect())?;


        Ok(())
    }
}

impl RenderText for Fader {
    fn render_text(
            &self,
            ttf: &sdl2::ttf::Sdl2TtfContext,
            canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
            font_path: &'static str,
        ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        if (self.is_hovered && self.display_on_hover) || !self.display_on_hover {
            let mut font = ttf.load_font(font_path, 12)?;
            font.set_style(sdl2::ttf::FontStyle::NORMAL);

            let surface = font
                .render(&format!("{:.2}", self.value()))
                .solid(Color::RGB(200, 200, 200))
                .map_err(|e| e.to_string())?;
            let label_tex = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;


            let TextureQuery { width, height, .. } = label_tex.query();
            let rect = self.bounds();
            let canvas_size = canvas.output_size()?;
            let x =  match canvas_size.0 as i32 > rect.x + width as i32 {
                true => rect.x as i32,
                false => (canvas_size.0 - width) as i32,
            };

            canvas.copy(
                &label_tex,
                None,
                rect!(
                    x,
                    rect.y.saturating_sub(height as i32),
                    width, 
                    height
                ),
            )?;
        }
        Ok(())
    }
}