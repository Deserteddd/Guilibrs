mod handler;
pub mod widget;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::handler::{EventHandler, HInstruction};
use crate::widget::{Textbox, Button};


macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

const DEFAULTFONT: &'static str = "./Courier_Prime.ttf";
const BACKROUNDCOLOR: Color = Color::RGB(40, 40, 40);
pub enum GuiEvent<T: Copy>{
  Quit,
  Custom(T),
  KeyPress(u8),
}
pub struct GUI<T> 
where T: Copy
{
  ttf_context: Sdl2TtfContext,
  font: &'static str,
  canvas: Canvas<Window>,
  handler: EventHandler,
  buttons: Vec<Button<T>>,
  textboxes: Vec<Textbox>,
  backround: Backround,
}
impl<T> GUI<T> 
  where T: Copy
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder::new()
  }

  pub fn tick(&mut self) -> Option<GuiEvent<T>> {
    if let Some(event) = self.handler.poll(&Self::get_bounds(&self)) {
      match event {
        HInstruction::Quit {..} => return Some(GuiEvent::Quit),
        
        HInstruction::Escape => return Some(GuiEvent::Quit),
        HInstruction::Return => {
          self.deselect_textboxes();
          return Some(GuiEvent::KeyPress(13))
        },
        HInstruction::PushChar(c) =>  {
          self.textboxes.iter_mut().for_each(|tb| if tb.is_active() {tb.push(c as char)});
          return Some(GuiEvent::KeyPress(c))
        },
        HInstruction::PopChar => {
          self.textboxes.iter_mut().for_each(|tb| if tb.is_active() {tb.pop_char();});
          return Some(GuiEvent::KeyPress(8));
        }
        HInstruction::Hover(u) => {
          match self.which_widget(u) {
            (WidgetType::Button, idx) => self.buttons[idx].is_hovered(true),
            _ => {},
          }
        },
        HInstruction::UnHover(u) => {
          match self.which_widget(u) {
            (WidgetType::Button, idx) => self.buttons[idx].is_hovered(false),
            _ => {},
          }
        },
        HInstruction::Click(u) => {
            self.deselect_textboxes();
            match self.which_widget(u) {
              (WidgetType::Button, idx) => return Some(GuiEvent::Custom(self.buttons[idx].click())),
              (WidgetType::Textbox, idx) => {
                if self.textboxes[idx].is_clickable() {
                  self.textboxes[idx].set_active(true);
                }
              },
            }
        },
      }
    }
    None
  }

  pub fn draw(&mut self) -> Result<(), String> {
    self.backround.render(&mut self.canvas)?;
    for i in self.buttons.iter() {
      i.render(&mut self.canvas)?;
      i.render_text(&self.ttf_context, &mut self.canvas, self.font)?;

    }
    for i in self.textboxes.iter() {
      i.render(&mut self.canvas)?;
      i.render_text(&self.ttf_context, &mut self.canvas, self.font)?;
    }

    self.canvas.present();
    Ok(())
  }
  pub fn textboxes(&self) -> std::slice::Iter<Textbox>{
    self.textboxes.iter()
  }
  pub fn get_input(&self, idx: usize) -> String {
    if idx >= self.textboxes.len() { panic!("get_input: Invalid textbox index")};
    self.textboxes[idx].to_string()
  }

  pub fn set_textbox_content(&mut self, idx: usize, content: String) {
    if idx >= self.textboxes.len() { return };
    if let Some(textbox) = self.textboxes.iter_mut().nth(idx) {
      textbox.set_content(content);
    }
  }

  pub fn push_to_textbox(&mut self, idx: usize, c: char) {
    match c as u8{
      8 => {self.textboxes[idx].pop_char();},
      _ => self.textboxes[idx].push(c),
    };
  }

  pub fn pop_from_textbox(&mut self, idx: usize) -> Option<char> {
    self.textboxes[idx].pop_char()
  } 

  pub fn clear_textbox(&mut self, idx: usize) {
    self.textboxes[idx].clear();
  }

  fn get_bounds(&self) -> Vec<Rect>{
    let mut bounds = self.buttons
      .iter()
      .map(|button| button.bounds())
      .collect::<Vec<Rect>>();
    bounds.append(&mut self.textboxes
      .iter()
      .map(|textbox| textbox.rect())
      .collect::<Vec<Rect>>()
      );
    assert_eq!(bounds.len(), self.buttons.len() + self.textboxes.len());
    bounds
  }

  fn which_widget(&self, idx: usize) -> (WidgetType, usize) {
    let buttons = self.buttons.len();
    let textboxes = self.textboxes.len();
    if buttons + textboxes == 0 {
      panic!("GUI::which_widget() called when program doesn't contain any")
    };
    if idx < buttons && buttons > 0 {
      (WidgetType::Button, idx)
    } else if idx - buttons < textboxes{
      (WidgetType::Textbox, idx - buttons)
    } else {
      panic!()
    }
  }

  fn deselect_textboxes(&mut self) {
    self.textboxes.iter_mut().for_each(|tb| {
      tb.set_active(false);
    })
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WidgetType{
  Button,
  Textbox
}

// GuiBuilder
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiBuilder<T> 
where T: Copy
{
  window_size: (u32, u32),
  backround_color: Color,
  window_title: &'static str,
  buttons: Vec<Button<T>>,
  textboxes: Vec<Textbox>,
  font: &'static str,
}
impl<T> GuiBuilder<T>
  where T: Copy
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder {
      window_size: (800, 600),
      backround_color: BACKROUNDCOLOR,
      window_title: "",
      buttons: vec![],
      textboxes: vec![],
      font: DEFAULTFONT,
    }
  }
  pub fn color_rgb(mut self, rgb: (u8, u8, u8)) -> GuiBuilder<T> {
    self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
    self
  }
  pub fn title(mut self, s: &'static str) -> GuiBuilder<T> {
    self.window_title = s;
    self
  }
  pub fn buttons(mut self, buttons: Vec<Button<T>>) -> GuiBuilder<T> {
    self.buttons = buttons;
    self
  }
  pub fn textboxes(mut self, tb: Vec<Textbox>) -> GuiBuilder<T> {
    self.textboxes = tb;
    self
  }
  pub fn font(mut self, s: &'static str) -> GuiBuilder<T> {
    self.font = s;
    self
  }
  pub fn size(mut self, w: u32, h: u32) -> GuiBuilder<T> {
    self.window_size.0 = w;
    self.window_size.1 = h;
    self
  }
  pub fn build(self) -> Result<GUI<T>, String>{

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let canvas = sdl_context.video()?
      .window(&self.window_title, self.window_size.0, self.window_size.1)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?
      .into_canvas()
      .build()
      .map_err(|e| e.to_string())?;
    return Ok(GUI {
      ttf_context: ttf_context,
      font: self.font,
      canvas: canvas,
      handler: EventHandler::new(&sdl_context)?,
      buttons: self.buttons,
      textboxes: self.textboxes,
      backround: Backround::new(
        self.backround_color,
        self.window_size.0,
        self.window_size.1,
      ),
    });
  }
}

// Backround
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Backround {
  pub color: Color,
  rect: Rect
}
impl Backround {
  pub fn new(color: Color, w: u32, h: u32) -> Self {
    Backround { 
      color: color,
      rect: rect!(0, 0, w, h),
    }
  }
}
impl Render for Backround {
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(self.color);
    canvas.fill_rect(self.rect)?;
    Ok(())
  }
}

pub trait Render {
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}
pub trait Widget<T> {
  fn click(&self) -> T;
  fn set_label(&mut self, s: &'static str);
}
pub trait RenderText {
  fn render_text(&self, ttf: &Sdl2TtfContext, canvas: &mut Canvas<Window>, font: &'static str, )
  -> Result<(), String>;
}