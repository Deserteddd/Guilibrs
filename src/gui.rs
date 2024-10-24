use crate::handler::{EventHandler, HandlerEvent};
use crate::textfield::TextField;
use crate::button::Button;

use crate::WidgetType;
use crate::Render;
use crate::RenderText;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

macro_rules! _rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

const DEFAULTFONT: &'static str = "./Courier_Prime.ttf";
const BACKROUNDCOLOR: Color = Color::RGB(40, 40, 40);

pub enum GuiEvent<T> {
    Quit,
    Callback(T),
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
    textboxes: Vec<TextField>,
}
impl<T> GUI<T>
where
    T: Copy,
    T: Default,
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
                GuiEvent::None
            },
            HandlerEvent::TextInput(text) => {
                self.textboxes.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.push(text.clone())
                    }
                });
                GuiEvent::None
            },
            HandlerEvent::PopChar => {
                self.textboxes.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.pop_char();
                    }
                });
                GuiEvent::None
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
                        return GuiEvent::Callback(self.buttons[idx].click());
                    }
                    (WidgetType::TextField, idx) => {
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
                self.switch_active_textbox();
                GuiEvent::None
            }
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(BACKROUNDCOLOR);
        self.canvas.clear();
        for btn in self.buttons.iter() {
            btn.render(&mut self.canvas)?;
            btn.render_text(&self.ttf_context, &mut self.canvas, self.font)?;
        }
        for tb in self.textboxes.iter() {
            tb.render(&mut self.canvas)?;
            tb.render_text(&self.ttf_context, &mut self.canvas, self.font)?;
        }

        self.canvas.present();
        Ok(())
    }

    pub fn textfields(&self) -> std::slice::Iter<TextField> {
        self.textboxes.iter()
    }

    pub fn textfields_mut(&mut self) -> std::slice::IterMut<TextField> {
        self.textboxes.iter_mut()
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
        self.textboxes[idx].push(c.to_string());
    }

    pub fn pop_from_textbox(&mut self, idx: usize) -> Option<char> {
        self.textboxes[idx].pop_char()
    }

    pub fn clear_textbox(&mut self, idx: usize) {
        self.textboxes[idx].clear();
    }

    fn switch_active_textbox(&mut self) {
        let first_clickable = self.textboxes
            .iter()
            .position(|tb| tb.is_clickable());

        if first_clickable.is_none() {
            return
        }
        let first_clickable = first_clickable.unwrap();

        let active = self.textboxes
            .iter()
            .position(|tb| tb.is_active());

        if active.is_none() {
            self.textboxes[first_clickable].set_active(true);
            return
        }
        let active = active.unwrap();

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
            (WidgetType::TextField, idx - buttons)
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
    textboxes: Vec<TextField>,
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
    pub fn textfields(mut self, tb: Vec<TextField>) -> GuiBuilder<T> {
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
        });
    }
}