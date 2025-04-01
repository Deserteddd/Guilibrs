use crate::{GuiEvent, BACKROUNDCOLOR, DEBUG};
use crate::handler::{EventHandler, HandlerEvent};
use crate::panel::Panel;
use crate::widgets::{Button, Fader, TextField, WidgetData};

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

use std::collections::HashMap;

pub struct GUI<T>
where
    T: Copy,
{
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    backround_color: Color,
    handler: EventHandler,
    panels: HashMap<&'static str, Panel<T>>,
    active_panels: Vec<&'static str>,
    active_widget: Option<WidgetData>,
}
impl<T> GUI<T>
where
    T: Copy + Default,
{
    pub fn new() -> GuiBuilder<T> {
        GuiBuilder::new()
    }

    pub fn poll(&mut self) -> GuiEvent<T> {
        let event = self.handler.poll_blocking(&mut self.panels, &self.active_panels);
        if event != HandlerEvent::None && unsafe {DEBUG}{
            println!("{:?}", event);
        }
        match event {
            HandlerEvent::None => return GuiEvent::None,
            HandlerEvent::Quit { .. } => return GuiEvent::Quit,
            HandlerEvent::Escape => self.deselect_all(),
            HandlerEvent::ClickBackround => self.deselect_all(),
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
            HandlerEvent::PopChar => {
                self.pop_active_textfield();
            },
            HandlerEvent::Hover(widget) => {
                self.hover_widget(widget);
            },
            HandlerEvent::HoverDropdown(widget, x, y) => {
                self.panels
                    .get_mut(widget.0)
                    .unwrap()
                    .hover_dropdown(widget.2, x, y);
            }
            HandlerEvent::UnHover(widget) => {
                self.unhover_widget(widget);
            },
            HandlerEvent::Drag(widget, x, y) => {
                if let Some(val) = self.drag(widget, x, y) {
                    return GuiEvent::FaderUpdate(widget.0, widget.2, val);
                }
            },
            HandlerEvent::Click(widget) => {
                if let Some(old_active) = self.active_widget {
                    if old_active.0 != widget.0 {
                        self.panels
                            .get_mut(old_active.0)
                            .unwrap()
                            .deselect(old_active.1, old_active.2);
                    }
                }
                self.active_widget = Some(widget);
                if let Some(cb) = self.panels
                    .get_mut(widget.0)
                    .unwrap()
                    .click(widget) {
                    return cb
                }
                
            },
            HandlerEvent::Return => {
                if let Some(widget) = self.active_widget {
                    println!("Enter");
                    if let Some(cb) = self.panels.get_mut(widget.0).unwrap().click(widget) {
                        return cb;
                    }
                }
            },
            HandlerEvent::Tab => {
                let panel = match self.active_widget {
                    Some(w) => w.0,
                    None => self.panels.keys().nth(0).unwrap()
                };
                self.active_widget = Some(self.panels
                    .get_mut(panel)
                    .unwrap()
                    .next_widget()
                )
            },
            HandlerEvent::ShitTab => {
                if let Some(widget) = self.active_widget {
                    self.active_widget = self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .previous_widget()
                    
                }
            },
            HandlerEvent::ArrowKey(dir) => {
                if let Some(widget) = self.active_widget {
                    if let Some(event) = self.panels
                        .get_mut(widget.0)
                        .unwrap()
                        .arrow_key(widget.1, widget.2, dir) {
                            return event
                        }
                }
            },
        }
        GuiEvent::None
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.backround_color);
        self.canvas.clear();
        for panel_name in self.panels.keys() {
            if self.active_panels.contains(panel_name) {
                self.panels[panel_name].draw(&mut self.canvas, &self.ttf_context)?;
            }
        }
        self.canvas.present();
        Ok(())
    }

    pub fn textfields(&self) -> std::slice::Iter<TextField> {
        self.panels.values().nth(0).unwrap().textfields.iter()
    }

    pub fn panel_textfields(&self, panel: &'static str) -> std::slice::Iter<TextField> {
        self.panels[panel].textfields.iter()
    }

    pub fn set_backround_color(&mut self, rgb: (u8, u8, u8)) {
        self.backround_color = Color::RGB(rgb.0, rgb.1, rgb.2);
    }

    pub fn panel_set_textfield_content(&mut self, panel: &'static str, idx: usize, content: String) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .set_textfield_content(idx, content);
    }

    pub fn set_textfield_content(&mut self, idx: usize, content: String) {
        self.panels
            .values_mut()
            .nth(0)
            .unwrap()
            .set_textfield_content(idx, content);
    }

    pub fn panel_set_fader_value(&mut self, panel: &'static str, fader: usize, value: f32) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .faders.iter_mut().nth(fader)
            .expect(&format!("Fader {} doesn't exist in panel '{}'", fader, panel))
            .set_fader_value(value);
    }

    pub fn set_fader_value(&mut self, fader: usize, value: f32) {
        self.panels
            .values_mut()
            .nth(0)
            .unwrap()
            .faders.iter_mut().nth(fader)
            .expect(&format!("Fader {} doesn't exist", fader))
            .set_fader_value(value);
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

    pub fn panel_push_to_textfield(&mut self, panel: &'static str, idx: usize, c: char) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .push_to_textfield(idx, c);
    }

    pub fn push_to_textfield(&mut self, idx: usize, c: char) {
        self.panels
            .values_mut()
            .nth(0)
            .unwrap()
            .push_to_textfield(idx, c);
    }

    pub fn show_panel(&mut self, panel: &'static str) {
        if !self.active_panels.contains(&panel) {
            self.active_panels.push(panel);
        }
    }

    pub fn hide_panel(&mut self, panel: &'static str) {
        if let Some(index) = self.active_panels.iter().position(|p| *p == panel) {
            self.active_panels.remove(index);
        }
    }

    pub fn panel_clear_textfield(&mut self, panel: &'static str, idx: usize) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
            .clear_textfield(idx);
    }

    pub fn clear_textfield(&mut self, idx: usize) {
        self.panels
            .values_mut()
            .nth(0)
            .unwrap()
            .clear_textfield(idx);
    }

    fn deselect_all(&mut self) {
        if let Some(widget) = self.active_widget {
            self.panels.get_mut(widget.0).unwrap().deselect(widget.1, widget.2);
            self.active_widget = None
        }
    } 

    fn unhover_widget(&mut self, widget: WidgetData) {
        self.panels
            .get_mut(widget.0)
            .unwrap()
            .unhover(widget.1, widget.2)

    }

    fn hover_widget(&mut self, widget: WidgetData) {
        self.panels
            .get_mut(widget.0)
            .unwrap()
            .hover(widget.1, widget.2);
    }

    fn drag(&mut self, widget: WidgetData, x: i32, y: i32) -> Option<f32> {
        self.panels
            .get_mut(widget.0)
            .unwrap()
            .drag(widget.1, widget.2, x, y)
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
    active_panels: Vec<&'static str>,
    buttons: Vec<Button<T>>,
    textfields: Vec<TextField>,
    faders: Vec<Fader>,
}
impl<T> GuiBuilder<T>
where
    T: Copy, T: Default
{
    pub fn new() -> GuiBuilder<T> {
        GuiBuilder {
            window_size: (800, 600),
            backround_color: BACKROUNDCOLOR,
            window_title: "",
            panels: HashMap::new(),
            active_panels: vec![],
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
    pub fn initial_panels(mut self, active: &[&'static str]) -> GuiBuilder<T> {
        self.active_panels.extend_from_slice(active);
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


        if self.panels.is_empty() {
            self.panels.insert(
                "default", 
                Panel::new("default", (0, 0), self.buttons, self.textfields, self.faders, vec![])
            );
        }

        if self.active_panels.is_empty() {
            for i in self.panels.iter() {
                self.active_panels.push(i.0);
            }
        }

        return Ok(GUI {
            ttf_context,
            canvas,
            backround_color: self.backround_color,
            handler: EventHandler::new(&sdl_context)?,
            panels: self.panels,
            active_panels: self.active_panels,
            active_widget: None,
        });
    }
}