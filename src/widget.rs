use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::{Canvas, TextureQuery};
use super::{Widget, Render, RenderText};

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Textbox{
  rect: Rect,
  font_size: u16,
  content: String,
  is_active: bool,
  clickable: bool,
}

impl Textbox {
  pub fn new(x: i32, y: i32, w: u32, h: u32) -> Textbox {
    Textbox { 
      rect: Rect::new(x, y, w, h),
      font_size: 24,
      content: String::new(),
      is_active: false, 
      clickable: false,
    }
  }
  pub fn clickable(mut self) -> Textbox {
    self.clickable = true;
    self
  }
  pub fn with_content(mut self, s: &'static str) -> Textbox {
    self.content = String::from(s);
    self
  }
  pub fn is_active(&self) -> bool {
    self.is_active
  }
  pub fn push(&mut self, c: char) {
    self.content.push(c)
  }
  pub fn pop_char(&mut self) -> Option<char> {
    self.content.pop()
  }
  pub fn set_active(&mut self, b: bool) {
    self.is_active = b
  }
  pub fn set_content(&mut self, s: String) {
    self.content = s
  }
  pub fn rect(&self) -> Rect {
    self.rect
  }
  pub fn is_clickable(&self) -> bool {
    self.clickable
  }
}

impl std::fmt::Display for Textbox {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.content)
  }
}

impl Widget<Option<String>> for Textbox {
  fn click(&self) -> Option<String> {
    if !self.clickable {
      None
    } else {
      Some(self.content.clone())
    }
  }
  fn set_label(&mut self, s: &'static str) {
      self.content = String::from(s);
  }
}
impl Render for Textbox {
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(self.rect)
  }
}

impl RenderText for Textbox {
  fn render_text(&self, ttf: &Sdl2TtfContext, canvas: &mut Canvas<Window>, font: &'static str) -> Result<(), String> 
  {
    if self.content.is_empty() {
      return Ok(())
    }
    let texture_creator = canvas.texture_creator();
    let mut font = ttf.load_font(font, self.font_size)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let surface = font
      .render(&self.content)
      .blended(Color::RGB(0, 0, 0))
      .map_err(|e| e.to_string())?;
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;
    let TextureQuery {width, height, ..} = texture.query();
    canvas.copy(&texture, None, rect!(
      self.rect.x + self.rect.w / 2 - width as i32 / 2, 
      self.rect.y + self.rect.h / 2 - height as i32 / 2, 
      width, 
      height
    ))?;
    
    Ok(())
  }
}

//Button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button<T>
where T: Copy
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
where T: Copy
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
where T: Copy
{
  fn click(&self) -> T {
    self.callback
  }
  fn set_label(&mut self, s: &'static str) {
    self.label = s;
  }
}
impl<T> Render for Button<T> 
where T: Copy {
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
impl<T> RenderText for Button<T> 
where T: Copy {
  fn render_text(
  &self, ttf: &Sdl2TtfContext, canvas: &mut Canvas<Window>, font: &'static str)
  -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let mut font = ttf.load_font(font, 0)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    let surface = font
      .render(&self.label)
      .blended(Color::RGB(0, 0, 0))
      .map_err(|e| e.to_string())?;
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;
    let TextureQuery {width, height, ..} = texture.query();

    canvas.copy(&texture, None, rect!(
      self.rect.x + self.rect.w / 2 - width as i32 / 2, 
      self.rect.y + self.rect.h / 2 - height as i32 / 2, 
      width, 
      height
    ))?;

    //Bounding box:
    
    Ok(())
  }
}
fn centered(rect: Rect, txt_w: u32, txt_h: u32) -> Rect {
  let x_middle = rect.w / 2; 
  let y_middle = rect.h / 2; 

  let r = Rect::new(rect.x, rect.y, rect.w as u32, rect.h as u32);
  r
}

//ButtonBuilder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonBuilder<T> {
  color: Option<Color>,
  rect: Option<Rect>,
  callback: Option<T>,
  label: &'static str,
  font_size: u16
}
impl<T> ButtonBuilder<T> 
where T: Copy
{
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder { color: None, rect: None, callback: None, label: " ", font_size: 24 }
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
    self.label = s;
    self
  }
  pub fn font_size(mut self, n: u16) -> ButtonBuilder<T> {
    self.font_size = n;
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