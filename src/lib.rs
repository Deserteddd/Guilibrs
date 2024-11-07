mod handler;
mod gui;
mod panel;
pub mod widgets;


pub use crate::gui::GUI;
pub use crate::panel::Panel;

use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const FONT: &'static str = "./Courier_Prime.ttf";
const BACKROUNDCOLOR: Color = Color::RGB(40, 40, 40);
static mut DEBUG: bool = false;

#[macro_export]
macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

pub enum GuiEvent<T> {
    Quit,
    Callback(&'static str, T),
    FaderUpdate(&'static str, usize, f32),
    DropdownUpdate(&'static str, usize, &'static str),
    None
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub trait RenderText {
    fn render_text(
        &self,
        ttf: &Sdl2TtfContext,
        canvas: &mut Canvas<Window>,
        font: &'static str,
    ) -> Result<(), String>;
}

fn bounding_box(rects: Vec<Rect>) -> Rect {
    if rects.is_empty() {
        return Rect::new(0, 0, 0, 0);
    }

    let min_x = rects.iter().map(|rect| rect.x).min().unwrap();
    let min_y = rects.iter().map(|rect| rect.y).min().unwrap();
    let max_x = rects.iter().map(|rect| rect.x + rect.width() as i32).max().unwrap();
    let max_y = rects.iter().map(|rect| rect.y + rect.height() as i32).max().unwrap();

    Rect::new(min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32)
}

fn in_bounds(rect: &Rect, x: i32, y: i32) -> bool {
    x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h
}