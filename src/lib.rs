//#![allow(warnings)]
use std::collections::VecDeque;
use sdl2::mouse::MouseButton;
use sdl2::surface::Surface;
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::{EventPump, Sdl};
use sdl2::video::Window;
use sdl2::render::{Canvas, TextureCreator, TextureQuery, Texture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

const FONT_PATH: &'static str = "C:/Windows/Fonts/Verdana.ttf";

pub struct GUI<T> 
where T: Copy,
{
  ttf_context: Sdl2TtfContext,
  canvas: Canvas<Window>,
  event_pump: EventPump,
  buttons: Vec<Button<T>>,
  backround: Backround,
}

impl<T> GUI<T> 
  where T: Copy
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder::new()
  }

  pub fn poll(&mut self, instruction_buffer: &mut VecDeque<T>) -> bool{
    let mut running = true;
    self.event_pump.poll_iter().for_each(|event| {
      match event {
        Event::Quit {..} => running = false,
        Event::KeyDown{keycode, ..} => match keycode { 
          Some(Keycode::Escape) => running = false,
          _ => {},
        },
        Event::MouseButtonDown {mouse_btn, x, y, ..} => {
          if mouse_btn == MouseButton::Left{
            for i in 0..self.buttons.len(){
              if self.buttons[i].in_bounds(x, y) {
                self.buttons[i].push();
              }
            }
          }
        },
        Event::MouseButtonUp {mouse_btn, x, y, ..} => {
          if mouse_btn == MouseButton::Left {
            for i in 0..self.buttons.len(){
              if self.buttons[i].is_pressed && self.buttons[i].in_bounds(x, y){
                instruction_buffer.push_back(self.buttons[i].release())
              } else {
                self.buttons[i].release();
              }
            }
          }
        }
        _ => {},
      };
    });
    running
  }

  pub fn draw(&mut self) -> Result<(), String> {
    let texture_creator = self.canvas.texture_creator();
    let mut font = self.ttf_context.load_font(FONT_PATH, 24)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    self.backround.render(&mut self.canvas)?;
    for i in &self.buttons {
      i.render(&mut self.canvas)?;
      let surface = font
        .render(i.label.string)
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
      let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
      let TextureQuery {width, height, ..} = texture.query();
      
      self.canvas.copy(&texture, None, rect!(i.rect.x+20, i.rect.y+100, width, height))?;
    }
    self.canvas.present();
    Ok(())
  }
}

// GuiBuilder
pub struct GuiBuilder<T> {
  window_size: (u32, u32),
  backround_color: Option<Color>,
  window_title: &'static str,
  buttons: Vec<Button<T>>,
}
impl<T> GuiBuilder<T>
  where T: Copy,
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder {
      window_size: (800, 600),
      backround_color: None,
      window_title: "",
      buttons: vec![],
    }
  }
  pub fn color(mut self, rgb: (u8, u8, u8)) -> GuiBuilder<T> {
    self.backround_color = Some(Color::RGB(rgb.0, rgb.1, rgb.2));
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

  pub fn size(mut self, w: u32, h: u32) -> GuiBuilder<T> {
    self.window_size.0 = w;
    self.window_size.1 = h;
    self
  }
  pub fn build(self) -> Result<GUI<T>, String>{
    if self.backround_color.is_none() {
      return Err("GUI::backround_color must be set".to_string());
    }
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
    let event_pump = sdl_context.event_pump().unwrap();
    return Ok(GUI {
      ttf_context: ttf_context,
      canvas: canvas,
      event_pump: event_pump,
      buttons: self.buttons,
      backround: Backround::new(
        self.backround_color.unwrap(),
        self.window_size.0,
        self.window_size.1,
      ),
    });
  }
}

//Button
#[derive(Clone)]
pub struct Button<T>{
  color: Color,
  rect: Rect,
  callback: T,
  is_pressed: bool,
  label: Text,
}
impl<T> Button<T> {
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder::new()
  }
}
impl<T> Widget<T> for Button<T> 
where T: Copy
{
  fn push(&mut self){
    self.color = self.color.invert();
    self.is_pressed = true;
  }
  fn release(&mut self) -> T {
    if self.is_pressed{
      self.color = self.color.invert();
    }
    self.is_pressed = false;
    self.callback
  }
  fn in_bounds(&self, x: i32, y: i32) -> bool {
    if x >= self.rect.x && x <= self.rect.x + self.rect.w &&
       y >= self.rect.y && y <= self.rect.y + self.rect.h 
    {
      return true;
    }
    false
  }
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(self.color);
    canvas.fill_rect(self.rect)?;
    Ok(())
  }
}

//ButtonBuilder
pub struct ButtonBuilder<T> {
  color: Option<Color>,
  rect: Option<Rect>,
  callback: Option<T>,
  label: Text
}
impl<T> ButtonBuilder<T> {
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder { color: None, rect: None, callback: None, label: Text::default() }
  }

  pub fn color(mut self, rgb: (u8, u8, u8)) -> ButtonBuilder<T> {
    self.color = Some(Color::RGB(rgb.0, rgb.1, rgb.2));
    self
  }
  pub fn rect(mut self, x:i32, y: i32, w: u32, h: u32) -> ButtonBuilder<T> {
    self.rect = Some(rect!(x, y, w, h));
    self
  }
  pub fn callback(mut self, cb: T) -> ButtonBuilder<T> {
    self.callback = Some(cb);
    self
  }
  pub fn label(mut self, s: &'static str) -> ButtonBuilder<T> {
    self.label.string = s;
    self
  }
  pub fn font(mut self, path: &'static str) -> ButtonBuilder<T> {
    self.label.font = path;
    self
  }
  pub fn build(self) -> Result<Button<T>, String> {
    if self.rect.is_none() {
      return Err("Button.rect must be set".to_string());
    }
    if self.color.is_none() {
      return Err("Button.color must be set".to_string());
    }
    return Ok(Button {
      color: self.color.unwrap(),
      rect: self.rect.unwrap(),
      callback: self.callback.unwrap(),
      is_pressed: false,
      label: self.label,
    });
  }
}

//Text
#[derive(Clone)]
pub struct Text {
  string: &'static str,
  font: &'static str,
}

impl Text {
  pub fn default() -> Text {
    let string = "Lorem ipsum";
    Text {
      font: "C:/Windows/Fonts/Verdana.ttf",
      string: string,
    }
  }
}

// Backround
#[derive(Debug)]
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
  fn push(&mut self);
  fn release(&mut self) -> T;
  fn in_bounds(&self, x: i32, y: i32) -> bool;
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}