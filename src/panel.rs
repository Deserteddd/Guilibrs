use core::panic;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;

use crate::widgets::{Widget, WidgetType, WidgetData, Button, Fader, TextField};
use crate::{Render, RenderText, DEBUG, bounding_box, in_bounds};

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
    pub font: &'static str
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

        Panel { name, bounds, buttons, textfields, faders, font: crate::DEFAULTFONT }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, active: Option<WidgetData>)
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
        if let Some(widget) = active {
            if widget.0 == self.name {
                let rect = match widget.1 {
                    WidgetType::Button => {
                        self.buttons[widget.2].visual_bounds()
                    },
                    WidgetType::Fader => {
                        self.faders[widget.2].visual_bounds()
                    },
                    WidgetType::TextField => {
                        self.textfields[widget.2].bounds()
                    }
                };
                canvas.set_draw_color(Color::RGB(80, 80, 80));
                canvas.draw_rect(rect)?;
            }
        }
        Ok(())
    }

    pub fn get_input(&self, idx: usize) -> String {
        if idx >= self.textfields.len() {
            panic!("get_input: Invalid textfield index")
        };
        self.textfields[idx].to_string()
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
        if let Some(btn) = self.buttons.iter().enumerate().find(|btn| in_bounds(&btn.1.visual_bounds(), x, y)) {
            return Some((self.name, WidgetType::Button, btn.0));
        }
        if let Some(tf) = self.textfields.iter().enumerate().find(|tf| in_bounds(&tf.1.visual_bounds(), x, y)) {
            return Some((self.name, WidgetType::TextField, tf.0));
        }
        if let Some(fd) = self.faders.iter().enumerate().find(|fd| in_bounds(&fd.1.visual_bounds(), x, y)) {
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
}
