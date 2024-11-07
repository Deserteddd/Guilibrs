use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureQuery}, ttf::Sdl2TtfContext, video::Window};

use crate::{rect, Render, RenderText};

use super::Widget;

//Button
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownButton
{
    options: Vec<&'static str>,
    rect: Rect,
    is_open: bool,
    active: usize,
    hovered: Option<usize>
}

impl DropdownButton {
    pub fn new(x: i32, y: i32) -> Self {
        DropdownButton {
            options: vec![],
            rect: Rect::new(x, y, 100, 20),
            is_open: false,
            active: 0,
            hovered: None
        }
    }

    pub fn options(mut self, options: Vec<&'static str>) -> Self {
        self.options = options;
        self
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.hovered = None;
        self.is_open = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn hover(&mut self, _x: i32, y: i32) {
        for i in 1..=self.options.len() {
            let lower = self.rect.y + (i as i32 * self.rect.h);
            let upper = self.rect.y + ((i+1) as i32 * self.rect.h);
            if y > lower && y <= upper {
                self.hovered = Some(i)
            }
        }
    }

    pub fn unhover(&mut self) {
        self.hovered = None;
    }

    pub fn click(&mut self) -> Option<&'static str> {
        if self.is_open && self.hovered.is_some(){
            self.active = self.hovered.unwrap();
            let result = Some(self.options[self.active-1]);
            self.close();
            return result
        } else if !self.is_open {
            self.open();
        } else {
            self.close();
        }
        None
    }
}

impl Widget for DropdownButton {
    fn bounds(&self) -> sdl2::rect::Rect {
        self.rect
    }
    fn shift(&mut self, x: i32, y: i32) {
        self.rect.x += x;
        self.rect.y += y;
    }

    fn visual_bounds(&self) -> sdl2::rect::Rect {
        if self.is_open {
            rect!(
                self.rect.x, self.rect.y, self.rect.w, self.rect.h * (self.options.len()+1) as i32
            )
        } else {
            self.rect
        }
    }
}


impl Render for DropdownButton {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(self.rect)?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(self.rect)?;
        if !self.is_open {
            return Ok(())
        }
        for i in 0..=self.options.len() {
            let rect = rect!(
                self.rect.x, self.rect.y + i as i32 * self.rect.h, self.rect.w, self.rect.h
            );
            if self.hovered.is_some() && self.hovered.unwrap() == i {
                canvas.set_draw_color(Color::RGB(200, 255, 200));
            } else {
                canvas.set_draw_color(Color::RGB(200, 200, 200));
            }
            canvas.fill_rect(rect)?;
            canvas.set_draw_color(Color::RGB(60, 60, 60));
            canvas.draw_line(
                (self.rect.x, self.rect.y + (i as i32 * self.rect.h)), 
                (self.rect.x+self.rect.w, self.rect.y + (i as i32 * self.rect.h))
            )?;
        }   
        Ok(())
    }
}

impl RenderText for DropdownButton
{
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font: &'static str,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let mut font = ttf.load_font(font, 16)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let surface = font
            .render(&self.options[self.active.saturating_sub(1)])
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();

        canvas.copy(
            &texture,
            None,
            rect!(
                self.rect.x + 10,
                self.rect.y + self.rect.h / 2 - height as i32 / 2,
                width,
                height
            ),
        )?;

        if !self.is_open {
            return Ok(())
        }

        for (idx, option) in self.options.iter().enumerate() {
            let surface = font
                .render(option)
                .blended(Color::RGB(0, 0, 0))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            let TextureQuery { width, height, .. } = texture.query();
            let rect = rect!(
                self.rect.x + 5, 
                self.rect.y + ((1 + idx) as i32 * self.rect.h) + 3, 
                width, 
                height
            );
            canvas.copy(
                &texture,
                None,
                rect
            )?;
        }



        if unsafe {crate::DEBUG}{
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(self.bounds())?;
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(self.visual_bounds())?;
        }
        Ok(())
    }
}