use crate::{Render, RenderText, rect};
use super::{Orientation, Widget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

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

    pub fn drag(&mut self, x: i32, y: i32) {
        match self.orientation {
            Orientation::Horizontal => {
                let x = x - self.position.0;
                self.value = x as f32 / self.length as f32;
                if self.value < 0.0 {
                    self.value = 0.0;
                } else if self.value > 1.0 {
                    self.value = 1.0;
                }
            },
            Orientation::Vertical => {
                let y = self.position.1 - y;
                self.value = y as f32 / self.length as f32;
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

impl Render for Fader {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let lerp = (self.value * self.length as f32) as i32;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));

        // Fader slit
        canvas.fill_rect(
            match self.orientation {
                Orientation::Horizontal => rect!(
                    self.position.0,
                    self.position.1-1,
                    self.length,
                    3
                ),
                Orientation::Vertical => rect!(
                    self.position.0 - 1,
                    self.position.1 - self.length,
                    3,
                    self.length
                )
            }
        )?;

        // Fader slit indicator
        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
        canvas.fill_rect(
            match self.orientation {
                Orientation::Horizontal => rect!(
                    self.position.0,
                    self.position.1 - 1,
                    lerp,
                    3
                ),
                Orientation::Vertical => rect!(
                    self.position.0 - 1,
                    self.position.1 - lerp,
                    3,
                    lerp
                )
            }
        )?;
        let knob = match self.orientation {
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
        };
        // fader knob
        canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
        canvas.fill_rect(knob)?;

        if self.is_hovered {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
        }        
        canvas.draw_rect(knob)?;


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
        if unsafe {crate::DEBUG} {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(self.bounds())?;
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(self.visual_bounds())?;
        }
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
            let x = match self.orientation {
                Orientation::Horizontal => match canvas_size.0 as i32 > rect.x + width as i32 {
                    true => rect.x as i32,
                    false => (canvas_size.0 - width) as i32,
                },
                Orientation::Vertical => rect.x + 25
            };

            let y = match self.orientation {
                Orientation::Horizontal => rect.y.saturating_sub(height as i32),
                Orientation::Vertical => rect.y
            };

            canvas.copy(
                &label_tex,
                None,
                rect!(
                    x,
                    y,
                    width, 
                    height
                ),
            )?;
        }

        Ok(())
    }
}