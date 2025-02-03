use crate::renderer::Renderer;
use crate::{GuiEvent, BACKROUNDCOLOR, DEBUG};
use crate::handler::{EventHandler, HandlerEvent};
use crate::panel::Panel;
use crate::widgets::{Button, Fader, TextField, WidgetData};

use sdl3::pixels::Color;
use sdl3::video::Window;
use sdl3::Error;

use std::collections::HashMap;

pub struct GUI<T>
where
    T: Copy,
{
    window: Window,
    renderer: Renderer,
    handler: EventHandler,
    panels: HashMap<&'static str, Panel<T>>,
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
        let event = self.handler.poll(&mut self.panels);
        if event != HandlerEvent::None && unsafe {DEBUG}{
            println!("{:?}", event);
        }
        match event {
            HandlerEvent::None => return GuiEvent::None,
            HandlerEvent::Quit => return GuiEvent::Quit,
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
                // println!("Hovering dropdown")
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
                    if let Some(cb) = self.panels.get_mut(widget.0).unwrap().click(widget) {
                        return cb;
                    }
                }
            },
            HandlerEvent::Tab => {
                let panel = match self.active_widget {
                    Some(w) => w.0,
                    None => { return GuiEvent::None }
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

    pub fn draw(&mut self) -> Result<(), Error> {
        self.renderer.render(&self.window)

    }

    pub fn textfields(&self, panel: &'static str) -> std::slice::Iter<TextField> {
        self.panels[panel].textfields.iter()
    }

    pub fn set_backround_color(&mut self, rgb: (u8, u8, u8)) {
        self.renderer.set_backround_color(rgb);
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

    pub fn clear_textfield(&mut self, panel: &'static str, idx: usize) {
        self.panels
            .get_mut(panel)
            .expect(&format!("Panel '{}' doesn't exist", panel))
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

    fn drag(&mut self, widget: WidgetData, x: f32, y: f32) -> Option<f32> {
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

    pub fn build(self) -> GUI<T> {
        let sdl_context = sdl3::init().map_err(|e| e.to_string())
            .expect("Failed to initialize SDL3");

        let window = sdl_context
            .video()
            .expect("Video subsystem creation failed")
            .window(&self.window_title, self.window_size.0, self.window_size.1)
            .position_centered()
            .build()
            .expect("Window creation failed");

        let renderer = Renderer::new(&window);

        let handler = EventHandler::new(&sdl_context)
            .expect("Event handler creation failed");

        return GUI {
            window,
            renderer,
            handler,
            panels: self.panels,
            active_widget: None,
        };
    }
}