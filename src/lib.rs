mod handler;
use std::collections::VecDeque;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::handler::{EventHandler, HInstruction};

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

pub struct GUI<T> 
where T: Copy, T: Default
{
  ttf_context: Sdl2TtfContext,
  font: &'static str,
  canvas: Canvas<Window>,
  handler: EventHandler,
  buttons: Vec<Button<T>>,
  backround: Backround,
}
impl<T> GUI<T> 
  where T: Copy, T: Default
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder::new()
  }

  pub fn poll(&mut self, instruction_buffer: &mut VecDeque<T>) -> bool{
    let mut running = true;
    self.handler.poll(&Self::button_bounds(&self)).iter().for_each(|event| {
      match event {
        HInstruction::Quit {..} => {
          println!("Quitting");
          running = false
        }
        HInstruction::Hover(u) => {
          self.buttons[*u].is_hovered = true;
        },
        HInstruction::UnHover(u) => {
          self.buttons[*u].is_hovered = false
        },
        HInstruction::Click(u) => {
          instruction_buffer.push_back(self.buttons[*u].click())
        }
      };
    });
    assert!(instruction_buffer.len() <= 1);
    running
  }

  pub fn draw(&mut self) -> Result<(), String> {
    let texture_creator = self.canvas.texture_creator();
    let mut font = self.ttf_context.load_font(self.font, 24)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    self.backround.render(&mut self.canvas)?;
    for i in self.buttons.iter() {
      i.render(&mut self.canvas)?;
      let surface = font
        .render(i.label)
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
      let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
      let TextureQuery {width, height, ..} = texture.query();
      
      self.canvas.copy(&texture, None, rect!(i.rect.x + 30, i.rect.y + 20, width, height))?;
    }
    self.canvas.present();
    Ok(())
  }

  fn button_bounds(&self) -> Vec<Rect>{
    self.buttons.iter().map(|button| button.rect).collect::<Vec<Rect>>()
  }
}

// GuiBuilder
pub struct GuiBuilder<T> 
where T: Copy, T: Default
{
  window_size: (u32, u32),
  backround_color: Option<Color>,
  window_title: &'static str,
  buttons: Vec<Button<T>>,
  font: &'static str,
}
impl<T> GuiBuilder<T>
  where T: Copy, T: Default
{
  pub fn new() -> GuiBuilder<T> {
    GuiBuilder {
      window_size: (800, 600),
      backround_color: None,
      window_title: "",
      buttons: vec![],
      font: "",
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
    if self.backround_color.is_none() {
      return Err("GUI::backround_color must be set".to_string());
    }
    if self.font.is_empty() {
      return Err("GUI::font must be set".to_string());
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
    return Ok(GUI {
      ttf_context: ttf_context,
      font: self.font,
      canvas: canvas,
      handler: EventHandler::new(&sdl_context)?,

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
#[derive(Debug, Clone, Copy)]
pub struct Button<T>
where T: Copy, T: Default
{
  color: Color,
  rect: Rect,
  callback: T,
  is_pressed: bool,
  is_hovered: bool,
  label: &'static str,
}
impl<T> Button<T> 
where T: Copy, T: Default
{
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder::new()
  }
}
impl<T> Widget<T> for Button<T> 
where T: Copy, T: Default
{
  fn click(&self) -> T {
    self.callback
  }
}
impl<T> Render for Button<T> 
where T: Copy, T: Default
{
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(match self.is_pressed || self.is_hovered{
      false => self.color,
      true => Color::RGBA(
        255,
        255,
        255,
        0
      )
    });
    canvas.fill_rect(self.rect)?;
    Ok(()) 
  }
}

//ButtonBuilder
pub struct ButtonBuilder<T> {
  color: Option<Color>,
  rect: Option<Rect>,
  callback: T,
  label: &'static str
}
impl<T> ButtonBuilder<T> 
where T: Copy, T: Default
{
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder { color: None, rect: None, callback: T::default(), label: " " }
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
    self.callback = cb;
    self
  }
  pub fn label(mut self, s: &'static str) -> ButtonBuilder<T> {
    self.label = s;
    self
  }
  pub fn build(mut self) -> Result<Button<T>, String> {
    if self.label.is_empty() {
      self.label = " ";
    }
    if self.rect.is_none() {
      return Err("Button.rect must be set".to_string());
    }
    if self.color.is_none() {
      return Err("Button.color must be set".to_string());
    }
    return Ok(Button {
      color: self.color.unwrap(),
      rect: self.rect.unwrap(),
      callback: self.callback,
      is_pressed: false,
      is_hovered: false,
      label: self.label,
    });
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
  fn click(&self) -> T;
}