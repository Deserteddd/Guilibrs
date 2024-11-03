use core::panic;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;

use crate::Direction;
use crate::widgets::{Widget, WidgetType, WidgetData, Button, Fader, TextField};
use crate::{bounding_box, in_bounds, GuiEvent, Render, RenderText, DEBUG};

#[derive(Debug, Clone, PartialEq)]
pub struct Panel<T> 
where
    T: Copy,
{
    pub name: &'static str,
    pub bounds: Rect,
    pub buttons: Vec<Button<T>>,
    pub textfields: Vec<TextField>,
    pub faders: Vec<Fader>,
    pub font: &'static str,
    widget_order: Vec<(WidgetType, usize)>,
    active: Option<usize>
}

impl<T> Panel<T> 
where 
    T: Copy + Default,
{
    pub fn new(
        name: &'static str,
        position: (i32, i32),
        mut buttons: Vec<Button<T>>,
        mut textfields: Vec<TextField>,
        mut faders: Vec<Fader>
    ) -> Panel<T> {
        buttons.iter_mut().for_each(|btn| btn.shift(position.0, position.1));
        textfields.iter_mut().for_each(|tf| tf.shift(position.0, position.1));
        faders.iter_mut().for_each(|fd| fd.shift(position.0, position.1));
        let bounds = bounding_box(
            [
                buttons.iter().map(|button| button.bounds()).collect::<Vec<Rect>>(),
                textfields.iter().map(|textfield| textfield.visual_bounds()).collect::<Vec<Rect>>(),
                faders.iter().map(|fader| fader.bounds()).collect::<Vec<Rect>>()
            ].concat()
        );
        let order = widget_order(buttons.clone(), textfields.clone(), faders.clone());
        Panel { 
            name, 
            bounds, 
            buttons, 
            textfields, 
            faders, 
            font: crate::DEFAULTFONT, 
            widget_order: order,
            active: None
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext)
    -> Result<(), String> {
        if unsafe { DEBUG } {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(self.bounds)?;
        }
        for button in &self.buttons {
            button.render(canvas)?;
            button.render_text(ttf, canvas, self.font)?;
        }
        for textfield in &self.textfields {
            textfield.render(canvas)?;
            textfield.render_text(ttf, canvas, self.font)?;
        }
        for fader in &self.faders {
            fader.render(canvas)?;
            fader.render_text(ttf, canvas, self.font)?;
        }
        if let Some(widget) = self.active {
            let widget = self.widget_order[widget];
            let rect = match widget.0 {
                WidgetType::Button => {
                    self.buttons[widget.1].visual_bounds()
                },
                WidgetType::Fader => {
                    self.faders[widget.1].visual_bounds()
                },
                WidgetType::TextField => {
                    self.textfields[widget.1].bounds()
                }
            };
            canvas.set_draw_color(Color::RGB(80, 80, 180));
            canvas.draw_rect(rect)?;
        }
        Ok(())
    }

    pub fn get_input(&self, idx: usize) -> String {
        if idx >= self.textfields.len() {
            panic!("get_input: Invalid textfield index")
        };
        self.textfields[idx].to_string()
    }

    pub fn arrow_key(&mut self, w_type: WidgetType, idx: usize, dir: Direction) -> Option<GuiEvent<T>> {
        match w_type {
            WidgetType::Fader => {
                match dir {
                    Direction::Up | Direction::Right => self.faders[idx].increment(),
                    Direction::Down | Direction::Left => self.faders[idx].decrement()
                }
                Some(GuiEvent::FaderUpdate(
                    self.name, 
                    idx, 
                    self.faders[idx].value()
                ))
            },
            _ => None
        }
    }

    pub fn deselect(&mut self, w_type: WidgetType, idx: usize) {
        match w_type {
            WidgetType::TextField => self.textfields[idx].set_active(false),
            _ => {}
        }
        self.active = None;
    }

    pub fn next_widget(&mut self) -> WidgetData {
        if self.active.is_none() {
            self.active = Some(0);
            self.activate_active_textfield();
            return (self.name, self.widget_order[0].0, self.widget_order[0].1);
        }
        let new = (self.active.unwrap() + 1) % self.widget_order.len();
        
        self.deselect_active_textfield();
        self.active = Some(new);
        self.activate_active_textfield();
        
        (self.name, self.widget_order[new].0, self.widget_order[new].1)
    }

    pub fn previous_widget(&mut self) -> Option<WidgetData> {
        let new = match self.active {
            Some(0) => self.widget_order.len()-1,
            Some(n) => n-1,
            None => return None
        };
        
        self.deselect_active_textfield();
        self.active = Some(new);
        self.activate_active_textfield();

        Some((self.name, self.widget_order[new].0, self.widget_order[new].1))
    } 

    pub fn set_textfield_content(&mut self, idx: usize, content: String) {
        if idx >= self.textfields.len() {
            return;
        };
        if let Some(textfield) = self.textfields.iter_mut().nth(idx) {
            textfield.set_content(content);
        }
    }

    pub fn push_to_textfield(&mut self, idx: usize, c: char) {
        self.textfields[idx].push(c.to_string());
    }

    pub fn push_to_active_textfields(&mut self, s: &str) {
        self.textfields.iter_mut().for_each(|textfield| {
            if textfield.is_active() {
                textfield.push(s.to_string());
            }
        });
    }

    pub fn pop_from_textfield(&mut self, idx: usize) -> Option<char> {
        self.textfields[idx].pop_char()
    }

    pub fn clear_textfield(&mut self, idx: usize) {
        self.textfields[idx].clear();
    }

    pub fn unhover(&mut self, w_type: WidgetType, idx: usize) {
        match w_type {
            WidgetType::Button => self.buttons[idx].is_hovered(false),
            WidgetType::Fader => self.faders[idx].is_hovered(false),
            _ => {}
        }    
    }

    pub fn hover(&mut self, w_type: WidgetType, idx: usize) {
        match w_type {
            WidgetType::Button => self.buttons[idx].is_hovered(true),
            WidgetType::Fader => self.faders[idx].is_hovered(true),
            _ => {}
        };
    }

    pub fn get_widget_data(&self, x: i32, y: i32) -> Option<WidgetData> {
        if let Some(btn) = self.buttons
            .iter()
            .enumerate()
            .find(|btn| in_bounds(&btn.1.visual_bounds(), x, y)) {
            return Some((self.name, WidgetType::Button, btn.0));
        }
        if let Some(tf) = self.textfields
            .iter()
            .enumerate()
            .find(|tf| in_bounds(&tf.1.visual_bounds(), x, y)) {
            return Some((self.name, WidgetType::TextField, tf.0));
        }
        if let Some(fd) = self.faders
            .iter()
            .enumerate()
            .find(|fd| in_bounds(&fd.1.visual_bounds(), x, y)) {
            return Some((self.name, WidgetType::Fader, fd.0));
        }
        None
    }

    pub fn drag(&mut self, w_type: WidgetType, idx: usize, x: i32, y: i32) -> Option<f32> {
        match w_type {
            WidgetType::Fader => {
                self.faders[idx].drag(x, y);
                Some(self.faders[idx].value())
            }
            _ => None
        }
    }

    pub fn click(&mut self, widget: WidgetData) -> Option<GuiEvent<T>> {
        self.active = Some(self.widget_order
            .iter()
            .enumerate()
            .find(|(_, w)| **w == (widget.1, widget.2))
            .unwrap()
            .0
        );
        match widget.1 {
            WidgetType::Button => Some(GuiEvent::Callback(self.name, self.buttons[widget.2].click())),
            WidgetType::TextField => {
                if self.textfields[widget.2].is_clickable() {
                    self.textfields[widget.2].set_active(true);
                }
                None
            },
            _ => None
        }
    }

    fn deselect_active_textfield(&mut self) {
        if let Some(active) = self.active {
            let active_widget = self.widget_order[active];
            if active_widget.0 == WidgetType::TextField {
                self.textfields[active_widget.1].set_active(false);
            }
        }
    }

    fn activate_active_textfield(&mut self) {
        if let Some(active) = self.active {
            let active_widget = self.widget_order[active];
            if active_widget.0 == WidgetType::TextField {
                self.textfields[active_widget.1].set_active(true);
            }
        }
    }
}

fn widget_order<T: Copy>(btns: Vec<Button<T>>, tfs: Vec<TextField>, fdrs: Vec<Fader>)
-> Vec<(WidgetType, usize)> {
    let mut widgets: Vec<(WidgetType, usize, i32, i32)> = btns
        .iter()
        .enumerate()
        .map(|(idx, btn)| (WidgetType::Button, idx, btn.visual_bounds().x, btn.visual_bounds().y))
        .collect();

    let mut tfs: Vec<(WidgetType, usize, i32, i32)> = tfs
        .iter()
        .enumerate()
        .map(|(idx, tf)| (WidgetType::TextField, idx, tf.visual_bounds().x, tf.visual_bounds().y))
        .collect();

    let mut fdrs: Vec<(WidgetType, usize, i32, i32)> = fdrs
        .iter()
        .enumerate()
        .map(|(idx, fdr)| (WidgetType::Fader, idx, fdr.visual_bounds().x, fdr.visual_bounds().y))
        .collect();
    widgets.append(&mut tfs);
    widgets.append(&mut fdrs);
    widgets.sort_unstable_by_key(|widget| (widget.3, widget.2));
    widgets.iter().map(|widget| (widget.0, widget.1)).collect()
}

