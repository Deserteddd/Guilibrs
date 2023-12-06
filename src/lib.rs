mod handler;
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

  pub fn poll(&mut self, instruction_buffer: &mut Vec<T>) -> bool{
    let mut running = true;
    self.handler.poll(&Self::get_bounds(&self)).iter().for_each(|event| {
      match event {
        HInstruction::Quit {..} => {
          running = false;
        }
        HInstruction::Escape | HInstruction::Return => {
          self.textboxes.iter_mut().for_each(|textbox| textbox.is_active = false);
        }
        HInstruction::Hover(u) => {
          match self.which_widget(*u).expect("Valid hover idx") {
            (WidgetType::Button, idx) => self.buttons[idx].is_hovered = true,
            _ => {},
          }
        },
        HInstruction::UnHover(u) => {
          match self.which_widget(*u).expect("Valid unhover idx") {
            (WidgetType::Button, idx) => self.buttons[idx].is_hovered = false,
            _ => {},
          }
        },
        HInstruction::Click(u) => {
            self.deselect_textboxes();
            match self.which_widget(*u).expect("Valid click idx") {
              (WidgetType::Button, idx) => instruction_buffer.push(self.buttons[idx].click()),
              (WidgetType::Textbox, idx) => self.textboxes[idx].is_active = true,
            }
        },
        HInstruction::Keypress(ch) => {
          println!("{:?}", ch);
          self.textboxes.iter_mut().for_each(|textbox| if textbox.is_active {
            match ch {
              '\u{8}' => {println!("pop"); textbox.content.pop();},
              _ => textbox.content.push(*ch),
            }
          });
        },
          
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
      self.canvas.copy(&texture, None, rect!(i.rect.x, i.rect.y, width, height))?;
    }
    for i in self.textboxes.iter() {
      i.render(&mut self.canvas)?;
    }

    self.canvas.present();
    Ok(())
  }

  pub fn get_input(&mut self, idx: usize) -> String {
    if idx >= self.textboxes.len() { panic!("get_input: Invalid textbox index")};
    self.textboxes[idx].content.clone()
  }

  fn get_bounds(&self) -> Vec<Rect>{
    let mut bounds = self.buttons
      .iter()
      .map(|button| button.rect)
      .collect::<Vec<Rect>>();
    bounds.append(&mut self.textboxes
      .iter()
      .map(|textbox| textbox.rect)
      .collect::<Vec<Rect>>()
      );
    assert_eq!(bounds.len(), self.buttons.len() + self.textboxes.len());
    bounds
  }

  fn which_widget(&self, idx: usize) -> Option<(WidgetType, usize)> {
    let buttons = self.buttons.len();
    let textboxes = self.textboxes.len();
    if idx < buttons {
      Some((WidgetType::Button, idx))
    } else if idx <= textboxes {
      Some((WidgetType::Textbox, idx-buttons))
    } else {
      None
    }
  }

  fn deselect_textboxes(&mut self) {
    self.textboxes.iter_mut().for_each(|tb| tb.is_active = false)
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
  backround_color: Option<Color>,
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
      backround_color: None,
      window_title: "",
      buttons: vec![],
      textboxes: vec![],
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
      textboxes: self.textboxes,
      backround: Backround::new(
        self.backround_color.unwrap(),
        self.window_size.0,
        self.window_size.1,
      ),
    });
  }
}

// Textbox
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Textbox{
  label: &'static str,
  rect: Rect,
  content: String,
  is_active: bool,
  clickable: bool,
}

impl Textbox {
  pub fn new(x: i32, y: i32, w: u32, h: u32) -> Textbox {
    Textbox { 
      label: "",
      rect: Rect::new(x, y, w, h),
      content: String::new(),
      is_active: false, 
      clickable: false,
    }
  }
  pub fn clickable(mut self) -> Textbox {
    self.clickable = true;
    self
  }
  pub fn label(mut self, s: &'static str) -> Textbox {
    self.label = s;
    self
  }
}
impl Widget<String> for Textbox {
  fn click(&self) -> String {
    self.content.clone()
  }
  fn set_label(&mut self, s: &'static str) {
      self.label = s;
  }
}
impl Render for Textbox {
  fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(self.rect)
  }
}

//Button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button<T>
where T: Copy
{
  color: Color,
  rect: Rect,
  callback: T,
  is_pressed: bool,
  is_hovered: bool,
  label: &'static str,
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
where T: Copy
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonBuilder<T> {
  color: Option<Color>,
  rect: Option<Rect>,
  callback: Option<T>,
  label: &'static str
}
impl<T> ButtonBuilder<T> 
where T: Copy
{
  pub fn new() -> ButtonBuilder<T> {
    ButtonBuilder { color: None, rect: None, callback: None, label: " " }
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