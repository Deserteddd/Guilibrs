
use crate::rect;
use super::Widget;

use sdl3::pixels::Color;
use sdl3::rect::Rect;

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