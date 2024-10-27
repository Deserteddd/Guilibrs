
use crate::{Render, RenderText};
use super::Widget;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

const DEFAULT_BTN_COL: Color = Color::RGB(85, 85, 85);

//Button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button<T>
where
    T: Copy,
{
    color: Color,
    label: &'static str,
    font_size: u16,
    rect: Rect,
    callback: T,
    is_pressed: bool,
    is_hovered: bool,
}

impl<T> Widget for Button<T> where T: Copy {
    fn shift(&mut self, x: i32, y: i32) {
        self.rect = rect!(self.rect.x + x, self.rect.y + y, self.rect.w, self.rect.h);
    }
    fn bounds(&self) -> Rect {
        self.rect
    }
}

impl<T> Button<T>
where
    T: Copy,
    T: Default
{
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Button<T> {
        Button {
            color: DEFAULT_BTN_COL,
            label: "",
            font_size: 24,
            rect: rect!(x, y, w, h),
            callback: T::default(),
            is_pressed: false,
            is_hovered: false,
        }
    }
    pub const fn click(&self) -> T {
        self.callback
    }
    pub const fn font_size(mut self, size: u16) -> Button<T> {
        self.font_size = size;
        self
    }
    pub const fn label(mut self, s: &'static str) -> Button<T> {
        self.label = s;
        self
    }
    pub const fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Button<T> {
        self.color = Color::RGB(r, g, b);
        self
    }
    pub const fn callback(mut self, cb: T) -> Button<T> {
        self.callback = cb;
        self
    }
    pub fn is_hovered(&mut self, b: bool) {
        self.is_hovered = b;
    }
}

impl<T> Render for Button<T>
where
    T: Copy,
{
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(match self.is_hovered {
            false => self.color,
            true => {
                let color = self.color.rgb();
                Color::RGBA(
                    color.0.saturating_mul(2),
                    color.1.saturating_mul(2),
                    color.2.saturating_mul(2), 
                    255)
            },
        });

        canvas.fill_rect(self.rect)?;
        Ok(())
    }
}

impl<T> RenderText for Button<T>
where
    T: Copy,
{
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font: &'static str,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let mut font = ttf.load_font(font, self.font_size)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let surface = font
            .render(&self.label)
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
                self.rect.x + self.rect.w / 2 - width as i32 / 2,
                self.rect.y + self.rect.h / 2 - height as i32 / 2,
                width,
                height
            ),
        )?;

        if unsafe {crate::DEBUG}{
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(self.bounds())?;
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(self.visual_bounds())?;
        }
        Ok(())
    }
}