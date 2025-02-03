use crate::rect;
use super::{TextAlign, Widget};
use sdl3::rect::Rect;

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