use crate::handler::{EventHandler, HandlerEvent};

use crate::{GuiEvent, BACKROUNDCOLOR};
use crate::panel::Panel;
use crate::widgets::{Button, Fader, TextField, WidgetType};

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

pub struct GUI<T>
where
    T: Copy,
{
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    backround_color: Color,
    handler: EventHandler,
    panel: Panel<T>,
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
        if event != HandlerEvent::None {
            // println!("{:?}", event);
        }
        match event {
            HandlerEvent::None => GuiEvent::None,
            HandlerEvent::Quit { .. } => GuiEvent::Quit,
            HandlerEvent::Escape => GuiEvent::Quit,
            HandlerEvent::Return => {
                self.deselect_textfields();
                GuiEvent::None
            },
            HandlerEvent::TextInput(text) => {
                self.panel.textfields.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.push(text.clone())
                    }
                });
                GuiEvent::None
            },
            HandlerEvent::PopChar => {
                self.panel.textfields.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.pop_char();
                    }
                });
                GuiEvent::None
            },
            HandlerEvent::Hover(u) => {
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => self.panel.buttons[idx].is_hovered(true),
                    (WidgetType::Fader, idx) => self.panel.faders[idx].is_hovered(true),
                    _ => {}
                }
                GuiEvent::None
            },
            HandlerEvent::UnHover(u) => {
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => self.panel.buttons[idx].is_hovered(false),
                    (WidgetType::Fader, idx) => self.panel.faders[idx].is_hovered(false),
                    _ => {}
                }
                GuiEvent::None
            },
            HandlerEvent::Drag(u, x, ..) => {
                match self.which_widget(u) {
                    (WidgetType::Fader, idx) => {
                        self.panel.faders[idx].drag(x);
                        return GuiEvent::FaderUpdate(idx, self.panel.faders[idx].value());
                    },
                    _ => GuiEvent::None
                }
            }
            HandlerEvent::Click(u) => {
                self.deselect_textfields();
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => {
                        return GuiEvent::Callback(self.panel.buttons[idx].click());
                    },
                    (WidgetType::TextField, idx) => {
                        if self.panel.textfields[idx].is_clickable() {
                            self.panel.textfields[idx].set_active(true);
                        }
                    },
                    _ => {}
                }
                GuiEvent::None
            },
            HandlerEvent::ClickBackround => {
                self.deselect_textfields();
                GuiEvent::None
            }
            HandlerEvent::TabPress => {
                self.switch_active_textfield();
                GuiEvent::None
            }
        }
    }

    pub fn swap_panel(&mut self, panel: Panel<T>) -> Panel<T> {
        self.panel.textfields.iter_mut()
            .filter(|tf| tf.is_password())
            .for_each(|tf| tf.clear());

        let old_buttons = std::mem::replace(&mut self.panel.buttons, panel.buttons);
        let old_textfields = std::mem::replace(&mut self.panel.textfields, panel.textfields);
        let old_faders = std::mem::replace(&mut self.panel.faders, panel.faders);
        Panel {
            buttons: old_buttons,
            textfields: old_textfields,
            faders: old_faders,
            font: crate::DEFAULTFONT
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.backround_color);
        self.canvas.clear();
        self.panel.draw(&mut self.canvas, &self.ttf_context)?;

        self.canvas.present();
        Ok(())
    }

    pub fn textfields(&self) -> std::slice::Iter<TextField> {
        self.panel.textfields.iter()
    }

    pub fn textfields_mut(&mut self) -> std::slice::IterMut<TextField> {
        self.panel.textfields.iter_mut()
    }


    pub fn get_input(&self, idx: usize) -> String {
        if idx >= self.panel.textfields.len() {
            panic!("get_input: Invalid textfield index")
        };
        self.panel.textfields[idx].to_string()
    }

    pub fn set_backround_color(&mut self, rgb: (u8, u8, u8)) {
        self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
    }

    pub fn set_textfield_content(&mut self, idx: usize, content: String) {
        if idx >= self.panel.textfields.len() {
            return;
        };
        if let Some(textfield) = self.panel.textfields.iter_mut().nth(idx) {
            textfield.set_content(content);
        }
    }

    pub fn push_to_textfield(&mut self, idx: usize, c: char) {
        self.panel.textfields[idx].push(c.to_string());
    }

    pub fn pop_from_textfield(&mut self, idx: usize) -> Option<char> {
        self.panel.textfields[idx].pop_char()
    }

    pub fn clear_textfield(&mut self, idx: usize) {
        self.panel.textfields[idx].clear();
    }

    fn switch_active_textfield(&mut self) {
        let first_clickable = self.panel.textfields
            .iter()
            .position(|tb| tb.is_clickable());

        if first_clickable.is_none() {
            return
        }
        let first_clickable = first_clickable.unwrap();

        let active = self.panel.textfields
            .iter()
            .position(|tb| tb.is_active());

        if active.is_none() {
            self.panel.textfields[first_clickable].set_active(true);
            return
        }
        let active = active.unwrap();

        let mut next_clickable = self.panel.textfields
            .iter()
            .enumerate()
            .skip(active+1)
            .find(|(_, tb)| tb.is_clickable())
            .map(|(idx, _)| idx);

        if next_clickable.is_none() {
            next_clickable = self.panel.textfields
                .iter()
                .position(|tb| tb.is_clickable());
        }

        self.panel.textfields[active].set_active(false);
        self.panel.textfields[next_clickable.unwrap()].set_active(true);
    }

    fn get_bounds(&self) -> Vec<Rect> {
        let mut bounds = self.panel
            .buttons
            .iter()
            .map(|button| button.bounds())
            .collect::<Vec<Rect>>();
        
        self.panel.textfields
            .iter()
            .for_each(|textfield| bounds.push(textfield.rect()));

        self.panel.faders
            .iter()
            .for_each(|fader| bounds.push(fader.bounds()));
        
        assert_eq!(
            bounds.len(),
            self.panel.buttons.len() + self.panel.textfields.len() + self.panel.faders.len()
        );
        bounds
    }

    fn which_widget(&self, idx: usize) -> (WidgetType, usize) {
        let buttons = self.panel.buttons.len();
        let textfields = self.panel.textfields.len();
        let faders = self.panel.faders.len();
        if buttons + textfields + faders == 0 {
            panic!("GUI::which_widget() called when program doesn't contain any")
        };
        if idx < buttons && buttons > 0 {
            (WidgetType::Button, idx)
        } else if idx - buttons < textfields {
            (WidgetType::TextField, idx - buttons)
        } else {
            (WidgetType::Fader, idx - buttons - textfields)
        }
    }

    fn deselect_textfields(&mut self) {
        self.panel.textfields.iter_mut().for_each(|tb| {
            tb.set_active(false);
        })
    }
}

// GuiBuilder
#[derive(Debug, Clone, PartialEq)]
pub struct GuiBuilder<T>
where
    T: Copy,
{
    window_size: (u32, u32),
    backround_color: Color,
    window_title: &'static str,
    panel: Panel<T>,
    buttons: Vec<Button<T>>,
    textfields: Vec<TextField>,
    faders: Vec<Fader>,
}
impl<T> GuiBuilder<T>
where
    T: Copy,
{
    pub const fn new() -> GuiBuilder<T> {
        GuiBuilder {
            window_size: (800, 600),
            backround_color: BACKROUNDCOLOR,
            window_title: "",
            panel: Panel::new(vec![], vec![], vec![]),
            buttons: vec![],
            textfields: vec![],
            faders: vec![],
        }
    }
    pub const fn color(mut self, rgb: (u8, u8, u8)) -> GuiBuilder<T> {
        self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
        self
    }
    pub const fn title(mut self, s: &'static str) -> GuiBuilder<T> {
        self.window_title = s;
        self
    }
    pub fn buttons(mut self, buttons: Vec<Button<T>>) -> GuiBuilder<T> {
        self.buttons = buttons;
        self
    }
    pub fn textfields(mut self, tb: Vec<TextField>) -> GuiBuilder<T> {
        self.textfields = tb;
        self
    }
    pub fn faders(mut self, faders: Vec<Fader>) -> GuiBuilder<T> {
        self.faders = faders;
        self
    }
    pub const fn size(mut self, w: u32, h: u32) -> GuiBuilder<T> {
        self.window_size.0 = w;
        self.window_size.1 = h;
        self
    }
    pub fn panel(mut self, panel: Panel<T>) -> GuiBuilder<T> {
        self.panel = panel;
        self
    }
    pub fn build(mut self) -> Result<GUI<T>, String> {
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

        if self.panel.is_empty() {
            self.panel = Panel::new(self.buttons, self.textfields, self.faders);
        }

        return Ok(GUI {
            ttf_context,
            canvas,
            backround_color: self.backround_color,
            handler: EventHandler::new(&sdl_context)?,
            panel: self.panel,
        });
    }
}