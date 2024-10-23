use crate::handler::{EventHandler, HandlerEvent};
use crate::widget::{Button, Textbox};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use crate::WidgetType;
use crate::Widget;
use crate::Render;
use crate::RenderText;

macro_rules! _rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

const DEFAULTFONT: &'static str = "./Courier_Prime.ttf";
const BACKROUNDCOLOR: Color = Color::RGB(40, 40, 40);

pub enum GuiEvent<T> {
    Quit,
    Custom(T),
    KeyPress(u8),
    None
}

pub struct GUI<T>
where
    T: Copy,
{
    ttf_context: Sdl2TtfContext,
    font: &'static str,
    canvas: Canvas<Window>,
    handler: EventHandler,
    buttons: Vec<Button<T>>,
    textboxes: Vec<Textbox>,
}
impl<T> GUI<T>
where
    T: Copy,
{
    pub fn new() -> GuiBuilder<T> {
        GuiBuilder::new()
    }

    pub fn poll(&mut self) -> GuiEvent<T> {
        let event = self.handler.poll(&Self::get_bounds(&self));
        match event {
            HandlerEvent::None => GuiEvent::None,
            HandlerEvent::Quit { .. } => GuiEvent::Quit,
            HandlerEvent::Escape => GuiEvent::Quit,
            HandlerEvent::Return => {
                self.deselect_textboxes();
                GuiEvent::KeyPress(13)
            },
            HandlerEvent::PushChar(c) => {
                self.textboxes.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.push(c as char)
                    }
                });
                GuiEvent::KeyPress(c)
            },
            HandlerEvent::PopChar => {
                self.textboxes.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.pop_char();
                    }
                });
                GuiEvent::KeyPress(8)
            },
            HandlerEvent::Hover(u) => {
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => self.buttons[idx].is_hovered(true),
                    _ => {}
                }
                GuiEvent::None
            },
            HandlerEvent::UnHover(u) => {
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => self.buttons[idx].is_hovered(false),
                    _ => {}
                }
                GuiEvent::None
            },
            HandlerEvent::Click(u) => {
                self.deselect_textboxes();
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => {
                        return GuiEvent::Custom(self.buttons[idx].click());
                    }
                    (WidgetType::Textbox, idx) => {
                        if self.textboxes[idx].is_clickable() {
                            self.textboxes[idx].set_active(true);
                        }
                    }
                }
                GuiEvent::None
            },
            HandlerEvent::ClickBackround => {
                self.deselect_textboxes();
                GuiEvent::None
            }
            HandlerEvent::TabPress => {
                // Switch to next clickable textbox
                let first_clickable = self.textboxes
                    .iter()
                    .position(|tb| tb.is_clickable());

                if first_clickable.is_none() {
                    return GuiEvent::None;
                }
                let first_clickable = first_clickable.unwrap();
                println!("first clickable: {}", first_clickable);

                let active = self.textboxes
                    .iter()
                    .position(|tb| tb.is_active());

                if active.is_none() {
                    self.textboxes[first_clickable].set_active(true);
                    return GuiEvent::None;
                }
                let active = active.unwrap();
                println!("Active: {}", active);

                let mut next_clickable = self.textboxes
                    .iter()
                    .enumerate()
                    .skip(active+1)
                    .find(|(_, tb)| tb.is_clickable())
                    .map(|(idx, _)| idx);

                if next_clickable.is_none() {
                    next_clickable = self.textboxes
                        .iter()
                        .position(|tb| tb.is_clickable());
                }

                self.textboxes[active].set_active(false);
                self.textboxes[next_clickable.unwrap()].set_active(true);

                GuiEvent::None
            }
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(BACKROUNDCOLOR);
        self.canvas.clear();
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
    pub fn textboxes(&self) -> std::slice::Iter<Textbox> {
        self.textboxes.iter()
    }
    pub fn get_input(&self, idx: usize) -> String {
        if idx >= self.textboxes.len() {
            panic!("get_input: Invalid textbox index")
        };
        self.textboxes[idx].to_string()
    }

    pub fn set_textbox_content(&mut self, idx: usize, content: String) {
        if idx >= self.textboxes.len() {
            return;
        };
        if let Some(textbox) = self.textboxes.iter_mut().nth(idx) {
            textbox.set_content(content);
        }
    }

    pub fn push_to_textbox(&mut self, idx: usize, c: char) {
        match c as u8 {
            8 => {
                self.textboxes[idx].pop_char();
            }
            _ => self.textboxes[idx].push(c),
        };
    }

    pub fn pop_from_textbox(&mut self, idx: usize) -> Option<char> {
        self.textboxes[idx].pop_char()
    }

    pub fn clear_textbox(&mut self, idx: usize) {
        self.textboxes[idx].clear();
    }

    fn get_bounds(&self) -> Vec<Rect> {
        let mut bounds = self
            .buttons
            .iter()
            .map(|button| button.bounds())
            .collect::<Vec<Rect>>();
        bounds.append(
            &mut self
                .textboxes
                .iter()
                .map(|textbox| textbox.rect())
                .collect::<Vec<Rect>>(),
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
        } else if idx - buttons < textboxes {
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

impl<T> Render for GUI<T> where T: Copy {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for i in self.buttons.iter() {
            i.render(canvas)?;
        }
        for i in self.textboxes.iter() {
            i.render(canvas)?;
        }
        Ok(())
    }
}

// GuiBuilder
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiBuilder<T>
where
    T: Copy,
{
    window_size: (u32, u32),
    backround_color: Color,
    window_title: &'static str,
    buttons: Vec<Button<T>>,
    textboxes: Vec<Textbox>,
    font: &'static str,
}
impl<T> GuiBuilder<T>
where
    T: Copy,
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
    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> GuiBuilder<T> {
        self.backround_color = Color::RGB(r, g, b);
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
    pub fn build(self) -> Result<GUI<T>, String> {
        let sdl_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let canvas = sdl_context
            .video()?
            .window(&self.window_title, self.window_size.0, self.window_size.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        return Ok(GUI {
            ttf_context,
            font: self.font,
            canvas,
            handler: EventHandler::new(&sdl_context)?,
            buttons: self.buttons,
            textboxes: self.textboxes,
            // backround: Backround::new(self.backround_color, self.window_size.0, self.window_size.1),
        });
    }
}