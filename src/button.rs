use crate::{Render, RenderText, Widget};
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
impl<T> Button<T>
where
    T: Copy,
{
    pub fn new() -> ButtonBuilder<T> {
        ButtonBuilder::new()
    }
    pub fn set_label(&mut self, s: &'static str) {
        self.label = s;
    }
    pub fn is_hovered(&mut self, b: bool) {
        self.is_hovered = b;
    }
    pub fn bounds(&self) -> Rect {
        self.rect
    }
}
impl<T> Widget<T> for Button<T>
where
    T: Copy,
{
    fn click(&mut self) -> T {
        self.callback
    }
    fn set_label(&mut self, s: &'static str) {
        self.label = s;
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
        let mut font = ttf.load_font(font, 24)?;
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

        //Bounding box:

        Ok(())
    }
}

//ButtonBuilder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonBuilder<T> {
    color: Option<Color>,
    rect: Option<Rect>,
    callback: Option<T>,
    label: &'static str,
    font_size: u16,
}
impl<T> ButtonBuilder<T>
where
    T: Copy,
{
    pub const fn new() -> ButtonBuilder<T> {
        ButtonBuilder {
            color: None,
            rect: None,
            callback: None,
            label: " ",
            font_size: 24,
        }
    }

    pub const fn color(mut self, rgb: (u8, u8, u8)) -> ButtonBuilder<T> {
        self.color = Some(Color::RGB(rgb.0, rgb.1, rgb.2));
        self
    }
    pub fn rect(mut self, x: i32, y: i32, w: u32, h: u32) -> ButtonBuilder<T> {
        self.rect = Some(rect!(x, y, w, h));
        self
    }
    pub const fn callback(mut self, cb: T) -> ButtonBuilder<T> {
        self.callback = Some(cb);
        self
    }
    pub const fn label(mut self, s: &'static str) -> ButtonBuilder<T> {
        self.label = s;
        self
    }
    pub const fn font_size(mut self, n: u16) -> ButtonBuilder<T> {
        self.font_size = n;
        self
    }
    pub fn build(mut self) -> Result<Button<T>, String> {
        if self.label.is_empty() {
            self.label = "";
        }
        if self.rect.is_none() {
            return Err("Button.rect must be set".to_string());
        }
        if self.color.is_none() {
            self.color = Some(DEFAULT_BTN_COL);
        }
        if self.callback.is_none() {
            return Err("Button.callback must be set".to_string());
        }
        return Ok(Button {
            color: self.color.unwrap(),
            rect: self.rect.unwrap(),
            callback: self.callback.unwrap(),
            is_pressed: false,
            is_hovered: false,
            label: self.label,
            font_size: self.font_size,
        });
    }
}
