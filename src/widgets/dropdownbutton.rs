use sdl3::rect::Rect;
use crate::rect;

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

    pub fn hover(&mut self, _x: f32, y: f32) {
        for i in 1..=self.options.len() {
            let lower = self.rect.y + (i as i32 * self.rect.h);
            let upper = self.rect.y + ((i+1) as i32 * self.rect.h);
            if y > lower as f32 && y <= upper as f32 {
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
    fn bounds(&self) -> Rect {
        self.rect
    }
    fn shift(&mut self, x: i32, y: i32) {
        self.rect.x += x;
        self.rect.y += y;
    }

    fn visual_bounds(&self) -> Rect {
        if self.is_open {
            rect!(
                self.rect.x, self.rect.y, self.rect.w, self.rect.h * (self.options.len()+1) as i32
            )
        } else {
            self.rect
        }
    }
}
