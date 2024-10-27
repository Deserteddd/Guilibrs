use crate::handler::{EventHandler, HandlerEvent};

use crate::{GuiEvent, BACKROUNDCOLOR, DEBUG};
use crate::panel::Panel;
use crate::widgets::{Button, Fader, TextField, WidgetType};

use std::collections::HashMap;

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
    panels: HashMap<&'static str, Panel<T>>,
}
impl<T> GUI<T>
where
    T: Copy + Default,
{
    pub fn new() -> GuiBuilder<T> {
        GuiBuilder::new()
    }

    pub fn poll(&mut self) -> GuiEvent<T> {
        let event = self.handler.poll(&mut self.panels);
        if event != HandlerEvent::None && unsafe {DEBUG}{
            println!("{:?}", event);
        }
        match event {
            HandlerEvent::None => return GuiEvent::None,
            HandlerEvent::Quit { .. } => return GuiEvent::Quit,
            HandlerEvent::Escape => return GuiEvent::Quit,
            HandlerEvent::ToggleDebug => {
                unsafe { crate::DEBUG = !crate::DEBUG; }
            },
            HandlerEvent::TextInput(ref text) => {
                self.panels
                    .iter_mut()
                    .for_each(|panel| {
                        panel.1.push_to_active_textfields(text);
                    });
            },
            HandlerEvent::ClickBackround => {
                self.deselect_textfields();
            },
            HandlerEvent::PopChar => {
                self.pop_active_textfield();
            },
            HandlerEvent::Hover(widget) => {
                match widget.1 {
                    WidgetType::Button => self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .buttons[widget.2]
                        .is_hovered(true),
                    WidgetType::Fader => self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .faders[widget.2]
                        .is_hovered(true),
                    _ => {}
                };
            },
            HandlerEvent::UnHover(widget) => {
                match widget.1 {
                    WidgetType::Button => self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .buttons[widget.2]
                        .is_hovered(false),
                    WidgetType::Fader => self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .faders[widget.2]
                        .is_hovered(false),
                    _ => {}
                }
            },
            HandlerEvent::Drag(widget, x, y) => {
                match widget.1 {
                    WidgetType::Fader => {
                        self.panels
                            .get_mut(widget.0)
                            .unwrap()
                            .faders[widget.2]
                            .drag(x, y);

                        return GuiEvent::FaderUpdate(widget.0, widget.2, self.panels[widget.0].faders[widget.2].value());
                    },
                    _ => {}
                }
            },
            HandlerEvent::Click(widget) => {
                self.deselect_textfields();
                match widget.1 {
                    WidgetType::Button => {
                        return GuiEvent::Callback(widget.0, self.panels[widget.0].buttons[widget.2].click());
                    },
                    WidgetType::TextField => {
                        if self.panels[widget.0].textfields[widget.2].is_clickable() {
                            self.panels.get_mut(widget.0).unwrap().textfields[widget.2].set_active(true);
                        }
                    },
                    _ => {}
                }
            },
            HandlerEvent::Return => {
                self.deselect_textfields();
            },
            HandlerEvent::TabPress => {
                self.deselect_textfields();
            }
        }
        GuiEvent::None
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.backround_color);
        self.canvas.clear();
        for panel in self.panels.values() {
            panel.draw(&mut self.canvas, &self.ttf_context)?;
        }

        self.canvas.present();
        Ok(())
    }

    pub fn textfields(&self, panel: &'static str) -> std::slice::Iter<TextField> {
        self.panels[panel].textfields.iter()
    }

    pub fn set_backround_color(&mut self, rgb: (u8, u8, u8)) {
        self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
    }

    pub fn set_textfield_content(&mut self, panel: &'static str, idx: usize, content: String) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .set_textfield_content(idx, content);
    }

    pub fn pop_active_textfield(&mut self) {
        self.panels.iter_mut().for_each(|panel|
            panel.1.textfields.iter_mut().for_each(|tb| {
                if tb.is_active() {
                    tb.pop_char();
                }
            })
        );
    }

    pub fn push_to_textfield(&mut self, panel: &'static str, idx: usize, c: char) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .push_to_textfield(idx, c);
    }

    pub fn pop_from_textfield(&mut self, panel: &'static str, idx: usize) -> Option<char> {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .pop_from_textfield(idx)
    }

    pub fn clear_textfield(&mut self, panel: &'static str, idx: usize) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .clear_textfield(idx);
    }

    pub fn panel_bounds(&self) -> Vec<Rect> {
        self.panels.values().map(|panel| panel.bounds).collect()
    }

    fn deselect_textfields(&mut self) {
        self.panels.iter_mut().for_each(|panel|
            panel.1.textfields.iter_mut().for_each(|tb| {
                tb.set_active(false);
            })
        );
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
    panels: HashMap<&'static str, Panel<T>>,
    buttons: Vec<Button<T>>,
    textfields: Vec<TextField>,
    faders: Vec<Fader>,
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
            panels: HashMap::new(),
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
    pub fn panels(mut self, panels: &[Panel<T>]) -> GuiBuilder<T> {
        for panel in panels {
            self.panels.insert(panel.name, panel.clone());
        }
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
            canvas,
            backround_color: self.backround_color,
            handler: EventHandler::new(&sdl_context)?,
            panels: self.panels,
        });
    }
}