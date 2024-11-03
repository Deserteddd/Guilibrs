use crate::{Render, RenderText, rect};
use super::{TextAlign, Widget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextField {
    rect: Rect,
    label: &'static str,
    font_size: u16,
    content: String,
    is_active: bool,
    clickable: bool,
    transparent: bool,
    text_align: TextAlign,
    password: bool,
}

impl Widget for TextField {
    fn shift(&mut self, x: i32, y: i32) {
        self.rect = rect!(self.rect.x + x, self.rect.y + y, self.rect.w, self.rect.h);
    }
    fn bounds(&self) -> Rect {
        self.rect
    }
    fn visual_bounds(&self) -> Rect {
        if self.label.is_empty() || self.transparent {
            self.rect
        } else {
            rect!(
                self.rect.x,
                self.rect.y.saturating_sub(13),
                self.rect.w,
                self.rect.h + 13
            )
        }
    }
}

impl TextField {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> TextField {
        TextField {
            rect: rect!(x, y, w, h),
            label: "",
            font_size: 24,
            content: String::new(),
            is_active: false,
            clickable: false,
            transparent: false,
            text_align: TextAlign::Left(5),
            password: false,
        }
    }
    pub const fn get_label(&self) -> &str {
        self.label
    }
    pub const fn password(mut self) -> TextField {
        self.password = true;
        self
    }
    pub const fn label(mut self, s: &'static str) -> TextField {
        self.label = s;
        self
    }
    pub const fn clickable(mut self) -> TextField {
        self.clickable = true;
        self
    }
    pub const fn font_size(mut self, size: u16) -> TextField {
        self.font_size = size;
        self
    }
    pub const fn align(mut self, align: TextAlign) -> TextField {
        self.text_align = align;
        self
    }
    pub const fn transparent(mut self) -> TextField {
        self.transparent = true;
        self
    }
    pub const fn is_active(&self) -> bool {
        self.is_active
    }

    pub const fn is_password(&self) -> bool {
        self.password
    }
    pub const fn is_clickable(&self) -> bool {
        self.clickable
    }
    pub fn content(mut self, s: &str) -> TextField {
        self.content = s.to_string();
        self
    }
    pub fn push(&mut self, text: String) {
        self.content.push_str(text.as_str())
    }
    pub fn pop_char(&mut self) -> Option<char> {
        self.content.pop()
    }
    pub fn set_active(&mut self, b: bool) {
        if b &! self.clickable {
            return;
        }
        self.is_active = b
    }
    pub fn set_content(&mut self, s: String) {
        self.content = s
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn clear(&mut self) {
        self.content.clear();
    }    
}

impl std::fmt::Display for TextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Render for TextField {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if !self.transparent {
            canvas.set_draw_color(Color::RGB(200, 200, 200));
            canvas.fill_rect(self.rect)?;
        }
        if unsafe { crate::DEBUG } {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(self.bounds())?;
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(self.visual_bounds())?;
        }

        Ok(())
    }
}

impl RenderText for TextField {
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font_path: &'static str,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let secret_text = "*".repeat(self.content.len());
        if !self.content.is_empty() {
            let mut font = ttf.load_font(font_path, self.font_size)?;
            font.set_style(sdl2::ttf::FontStyle::NORMAL);
            canvas.set_clip_rect(Some(self.rect));
            let surface = font
                .render(match self.password {
                    true => &secret_text,
                    false => &self.content,
                })
                .blended(Color::RGB(0, 0, 0))
                .map_err(|e| e.to_string())?;
            let content_tex = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            let TextureQuery { width, height, .. } = content_tex.query();
            canvas.copy(
                &content_tex,
                None,
                match self.text_align {
                    TextAlign::Left(n) => rect!(
                        self.rect.x + n, 
                        self.rect.y + self.rect.h / 2 - height as i32 / 2,
                        width, 
                        height
                    ),
                    TextAlign::Right(n) => rect!(
                        self.rect.x + self.rect.w - width as i32 - n,
                        self.rect.y + self.rect.h / 2 - height as i32 / 2,
                        width,
                        height
                    ),
                    TextAlign::Center => rect!(
                        self.rect.x + self.rect.w / 2 - width as i32 / 2,
                        self.rect.y + self.rect.h / 2 - height as i32 / 2,
                        width,
                        height
                    ),
                }
            )?;
            canvas.set_clip_rect(None);
        }
        // Label
        if !self.label.is_empty() && !self.transparent {
            let mut font = ttf.load_font(font_path, 12)?;
            font.set_style(sdl2::ttf::FontStyle::NORMAL);

            let surface = font
                .render(&self.label)
                .blended(Color::RGB(200, 200, 200))
                .map_err(|e| e.to_string())?;
            let label_tex = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;


            let TextureQuery { width, height, .. } = label_tex.query();
            canvas.copy(
                &label_tex,
                None,
                rect!(
                    self.rect.x, 
                    self.rect.y.saturating_sub(height as i32),
                    width, 
                    height
                ),
            )?;
        }

        Ok(())
    }
}