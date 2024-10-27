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
    active_panel: Option<&'static str>,
}
impl<T> GUI<T>
where
    T: Copy + Default,
{
    pub fn new() -> GuiBuilder<T> {
        GuiBuilder::new()
    }

    fn active_panel(&mut self) -> Option<&mut Panel<T>> {
        self.panels.get_mut(self.active_panel?)
    }

    pub fn poll(&mut self) -> GuiEvent<T> {
        let event = self.handler.poll(&mut self.panels);
        if event != HandlerEvent::None && DEBUG{
            println!("{:?}", event);
        }

        // Events that return a GuiEvent which != GuiEvent::None
        match event {
            HandlerEvent::None => return GuiEvent::None,
            HandlerEvent::Quit { .. } => return GuiEvent::Quit,
            HandlerEvent::Escape => return GuiEvent::Quit,
            HandlerEvent::ActivePanel(panel) => {
                if let Some(previous) = self.active_panel() {
                    previous.unhover_buttons();
                }
                self.active_panel = panel;
            },

            _ => {}
        }
        if let Some(active) = self.active_panel {
            match event {
                HandlerEvent::TextInput(text) => {
                    self.panels
                        .get_mut(active)
                        .unwrap()
                        .textfields
                        .iter_mut()
                        .for_each(|tb| {
                            if tb.is_active() {
                                tb.push(text.clone())
                            }
                        });
                },
                HandlerEvent::PopChar => {
                    self.panels
                        .get_mut(active)
                        .unwrap()
                        .textfields
                        .iter_mut()
                        .for_each(|tb| {
                            if tb.is_active() {
                                tb.pop_char();
                            }
                        });
                },
                HandlerEvent::Hover(u) => {
                    match self.which_widget(u) {
                        (WidgetType::Button, idx) => self.panels
                            .get_mut(active)
                            .unwrap()
                            .buttons[idx]
                            .is_hovered(true),
                        (WidgetType::Fader, idx) => self.panels
                            .get_mut(active)
                            .unwrap()
                            .faders[idx]
                            .is_hovered(true),
                        _ => {}
                    }
                },
                HandlerEvent::UnHover(u) => {
                    match self.which_widget(u) {
                        (WidgetType::Button, idx) => self.panels
                            .get_mut(active)
                            .unwrap()
                            .buttons[idx]
                            .is_hovered(false),
                        (WidgetType::Fader, idx) => self.panels
                            .get_mut(active)
                            .unwrap()
                            .faders[idx]
                            .is_hovered(false),
                        _ => {}
                    }
                },
                HandlerEvent::Drag(u, x, ..) => {
                    match self.which_widget(u) {
                        (WidgetType::Fader, idx) => {
                            self.panels
                                .get_mut(active)
                                .unwrap()
                                .faders[idx]
                                .drag(x);
                            return GuiEvent::FaderUpdate(idx, self.panels[active].faders[idx].value());
                        },
                        _ => {}
                    }
                }
                HandlerEvent::Click(u) => {
                    self.deselect_textfields();
                    match self.which_widget(u) {
                        (WidgetType::Button, idx) => {
                            return GuiEvent::Callback(self.panels[active].buttons[idx].click());
                        },
                        (WidgetType::TextField, idx) => {
                            if self.panels[active].textfields[idx].is_clickable() {
                                self.panels.get_mut(active).unwrap().textfields[idx].set_active(true);
                            }
                        },
                        _ => {}
                    }
                },
                HandlerEvent::Return => {
                    self.deselect_textfields();
                },
                HandlerEvent::ClickBackround => {
                    self.deselect_textfields();
                }
                // HandlerEvent::TabPress => {
                //     self.panels.get_mut(active).unwrap().textfields[idx].set_active(true);
                //     self.switch_active_textfield();
                // }
                _ => {}
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

    pub fn textfields(&self) -> std::slice::Iter<TextField> {
        self.panels[self.active_panel.unwrap_or("default")].textfields.iter()
    }

    // pub fn textfields_mut(&mut self) -> std::slice::IterMut<TextField> {
    //     self.panel.textfields.iter_mut()
    // }


    pub fn set_backround_color(&mut self, rgb: (u8, u8, u8)) {
        self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
    }

    pub fn set_textfield_content(&mut self, panel: &'static str, idx: usize, content: String) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .set_textfield_content(idx, content);
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

    // fn switch_active_textfield(&mut self) {
    //     let first_clickable = self.panel.textfields
    //         .iter()
    //         .position(|tb| tb.is_clickable());

    //     if first_clickable.is_none() {
    //         return
    //     }
    //     let first_clickable = first_clickable.unwrap();

    //     let active = self.panel.textfields
    //         .iter()
    //         .position(|tb| tb.is_active());

    //     if active.is_none() {
    //         self.panel.textfields[first_clickable].set_active(true);
    //         return
    //     }
    //     let active = active.unwrap();

    //     let mut next_clickable = self.panel.textfields
    //         .iter()
    //         .enumerate()
    //         .skip(active+1)
    //         .find(|(_, tb)| tb.is_clickable())
    //         .map(|(idx, _)| idx);

    //     if next_clickable.is_none() {
    //         next_clickable = self.panel.textfields
    //             .iter()
    //             .position(|tb| tb.is_clickable());
    //     }

    //     self.panel.textfields[active].set_active(false);
    //     self.panel.textfields[next_clickable.unwrap()].set_active(true);
    // }


    fn which_widget(&self, idx: usize) -> (WidgetType, usize) {
        let active = self.active_panel.expect("GUI::which_widget called while no panel active");
        let buttons = self.panels[active].buttons.len();
        let textfields = self.panels[active].textfields.len();
        let faders = self.panels[active].faders.len();
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
            active_panel: None,
            panels: self.panels,
        });
    }
}