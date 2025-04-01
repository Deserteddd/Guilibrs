use crate::widgets::{WidgetData, WidgetType};
use crate::{Panel, in_bounds, Direction};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::{EventPump, Sdl};

use std::collections::HashMap;

pub struct EventHandler {
    pump: EventPump,
    active_panel: Option<&'static str>,
    lmb_pressed_on: Option<WidgetData>,
    hovered: Option<WidgetData>,
    quit_on_escape: bool
}

impl EventHandler {
    pub fn new(context: &Sdl, quit_on_escape: bool) -> Result<EventHandler, String> {
        Ok(EventHandler {
            pump: context.event_pump()?,
            active_panel: None,
            hovered: None,
            lmb_pressed_on: None,
            quit_on_escape
        })
    }

    pub fn poll_blocking<T: Copy + Default>(&mut self, panels: &mut HashMap<&'static str, Panel<T>>, visible_panels: &Vec<&'static str>) -> HandlerEvent {
        match self.pump.wait_event() {
            Event::Quit { .. } => HandlerEvent::Quit,
            Event::TextInput { text, .. } => HandlerEvent::TextInput(text),
            Event::KeyDown { keycode, keymod, .. } => {
                self.parse_keycode(keycode, keymod)
            },
            Event::MouseMotion { x, y, .. } => {
                // If something is pressed, we are dragging it
                if let Some(widget_data) = self.lmb_pressed_on {
                    return HandlerEvent::Drag(widget_data, x, y)
                }

                // If we are not on any panel, we aren't hovering anything
                self.active_panel = panels
                    .iter()
                    .find(|panel| in_bounds(&panel.1.bounds, x, y) && visible_panels.contains(panel.0))
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
                if hovered.is_some() 
                && hovered.unwrap().1 == WidgetType::DropdownButton {
                    self.hovered = hovered;
                    return HandlerEvent::HoverDropdown(hovered.unwrap(), x, y);
                }
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

    const fn parse_keycode(&self, kc: Option<Keycode>, km: Mod) -> HandlerEvent {
        if let Some(keycode) = kc {
            return match keycode {
                Keycode::Backspace => HandlerEvent::PopChar,
                Keycode::Return    => HandlerEvent::Return,
                Keycode::F12       => HandlerEvent::ToggleDebug,
                Keycode::Right     => HandlerEvent::ArrowKey(Direction::Right),
                Keycode::Left      => HandlerEvent::ArrowKey(Direction::Left),
                Keycode::Up        => HandlerEvent::ArrowKey(Direction::Up),
                Keycode::Down      => HandlerEvent::ArrowKey(Direction::Down),
                Keycode::Tab       => match km.contains(Mod::LSHIFTMOD) || km.contains(Mod::RSHIFTMOD) {
                    true  => HandlerEvent::ShitTab,
                    false => HandlerEvent::Tab
                },
                Keycode::Escape    => match self.quit_on_escape {
                    true  => HandlerEvent::Quit,
                    false => HandlerEvent::Escape
                },
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
    HoverDropdown(WidgetData, i32, i32),
    UnHover(WidgetData),
    Click(WidgetData),
    Drag(WidgetData, i32, i32),
    ToggleDebug,
    Escape,
    Return,
    TextInput(String),
    ArrowKey(Direction),
    PopChar,
    ClickBackround,
    Tab,
    ShitTab,
    None
}


