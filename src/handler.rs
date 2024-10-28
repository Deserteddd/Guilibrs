use crate::widgets::WidgetData;
use crate::{Panel, in_bounds};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{EventPump, Sdl};

use std::collections::HashMap;

pub struct EventHandler {
    pump: EventPump,
    active_panel: Option<&'static str>,
    lmb_pressed_on: Option<WidgetData>,
    hovered: Option<WidgetData>,
}

impl EventHandler {
    pub fn new(context: &Sdl) -> Result<EventHandler, String> {
        Ok(EventHandler {
            pump: context.event_pump()?,
            active_panel: None,
            hovered: None,
            lmb_pressed_on: None,
        })
    }

    pub fn poll<T: Copy + Default>(&mut self, panels: &mut HashMap<&'static str, Panel<T>>) -> HandlerEvent {
        match self.pump.wait_event() {
            Event::Quit { .. } => HandlerEvent::Quit,
            Event::TextInput { text, .. } => HandlerEvent::TextInput(text),
            Event::KeyDown { keycode, .. } => {
                self.parse_keycode(keycode)
            },
            Event::MouseMotion { x, y, .. } => {
                // If something is pressed, we are dragging it
                match self.lmb_pressed_on {
                    Some(widget_data) => {
                        return HandlerEvent::Drag(widget_data, x, y)
                    }
                    None => {}
                }

                // If we are not on any panel, we aren't hovering anything
                self.active_panel = panels
                    .iter()
                    .find(|panel| in_bounds(&panel.1.bounds, x, y))
                    .map(|panel| *panel.0);
                if self.active_panel.is_none() {
                    if self.hovered.is_some() {
                        let hovered = self.hovered.unwrap();
                        self.hovered = None;
                        return HandlerEvent::UnHover(hovered);
                    }
                    return HandlerEvent::None
                }

                // We are hovering a panel
                let active_panel = self.active_panel.unwrap();
                let hovered = panels[active_panel].get_widget_data(x, y);
                if hovered.is_some() && self.hovered != hovered {
                    self.hovered = hovered;
                    return HandlerEvent::Hover(hovered.unwrap());
                } else if hovered.is_none() && self.hovered.is_some() {
                    let unhovered = self.hovered.unwrap();
                    self.hovered = None;
                    return HandlerEvent::UnHover(unhovered);
                }
                
                HandlerEvent::None
            },
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn != MouseButton::Left {
                    return HandlerEvent::None
                }
                if self.active_panel.is_none() {
                    return HandlerEvent::None
                }
                let active_panel = self.active_panel.unwrap();
                let widget_data = panels[active_panel].get_widget_data(x, y);

                self.lmb_pressed_on = widget_data;

                HandlerEvent::None
            },

            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                if mouse_btn != MouseButton::Left {
                    return HandlerEvent::None
                }
                if self.active_panel.is_some() && self.lmb_pressed_on.is_some() {
                    let widget = panels[self.active_panel.unwrap()].get_widget_data(x, y);
                    let pressed_on = self.lmb_pressed_on.unwrap();
                    self.lmb_pressed_on = None;
                    if widget == Some(pressed_on) {
                        return HandlerEvent::Click(widget.unwrap());
                    }
                    return HandlerEvent::UnHover(pressed_on);
                } else {
                    return HandlerEvent::ClickBackround
                }
            }
            _ => HandlerEvent::None,
        }
    }

    const fn parse_keycode(&self, kc: Option<Keycode>) -> HandlerEvent {
        if let Some(keycode) = kc {
            return match keycode {
                Keycode::Backspace => HandlerEvent::PopChar,
                Keycode::Tab => HandlerEvent::TabPress,
                Keycode::Return => HandlerEvent::Return,
                Keycode::Escape => HandlerEvent::Escape,
                Keycode::F12 => HandlerEvent::ToggleDebug,
                _ => HandlerEvent::None,
            }
        }
        HandlerEvent::None
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandlerEvent {
    Quit,
    Hover(WidgetData),
    UnHover(WidgetData),
    Click(WidgetData),
    Drag(WidgetData, i32, i32),
    ToggleDebug,
    Escape,
    Return,
    TextInput(String),
    PopChar,
    ClickBackround,
    TabPress,
    None
}