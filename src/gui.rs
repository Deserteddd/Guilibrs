use crate::handler::{EventHandler, HandlerEvent};
use crate::slider::Slider;
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
    NewSliderValue(usize, f32),
    None
}

pub struct Panel<T> 
where
    T: Copy,
{
    buttons: Vec<Button<T>>,
    textfields: Vec<TextField>,
    sliders: Vec<Slider>,
}

impl<T> Panel<T> 
where 
    T: Copy,
{
    pub fn new(
        buttons: Vec<Button<T>>,
        textfields: Vec<TextField>,
        sliders: Vec<Slider>
    ) -> Panel<T> {
        Panel { buttons, textfields, sliders }
    }
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
    textfields: Vec<TextField>,
    sliders: Vec<Slider>
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
                self.textfields.iter_mut().for_each(|tb| {
                    if tb.is_active() {
                        tb.push(text.clone())
                    }
                });
                GuiEvent::None
            },
            HandlerEvent::PopChar => {
                self.textfields.iter_mut().for_each(|tb| {
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
            HandlerEvent::Drag(u, x, ..) => {
                match self.which_widget(u) {
                    (WidgetType::Slider, idx) => {
                        self.sliders[idx].drag(x);
                        return GuiEvent::NewSliderValue(idx, self.get_slider_value(idx));
                    },
                    _ => GuiEvent::None
                }
            }
            HandlerEvent::Click(u) => {
                self.deselect_textfields();
                match self.which_widget(u) {
                    (WidgetType::Button, idx) => {
                        return GuiEvent::Callback(self.buttons[idx].click());
                    },
                    (WidgetType::TextField, idx) => {
                        if self.textfields[idx].is_clickable() {
                            self.textfields[idx].set_active(true);
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
                self.switch_active_textbox();
                GuiEvent::None
            }
        }
    }

    pub fn swap_panel(&mut self, panel: Panel<T>) -> Panel<T> {
        self.textfields.iter_mut()
            .filter(|tf| tf.is_password())
            .for_each(|tf| tf.clear());

        let old_buttons = std::mem::replace(&mut self.buttons, panel.buttons);
        let old_textfields = std::mem::replace(&mut self.textfields, panel.textfields);
        let old_sliders = std::mem::replace(&mut self.sliders, panel.sliders);
        Panel {
            buttons: old_buttons,
            textfields: old_textfields,
            sliders: old_sliders,
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(BACKROUNDCOLOR);
        self.canvas.clear();
        for btn in self.buttons.iter() {
            btn.render(&mut self.canvas)?;
            btn.render_text(&self.ttf_context, &mut self.canvas, self.font)?;
        }
        for tb in self.textfields.iter() {
            tb.render(&mut self.canvas)?;
            tb.render_text(&self.ttf_context, &mut self.canvas, self.font)?;
        }
        for sl in self.sliders.iter() {
            sl.render(&mut self.canvas)?;
        }

        self.canvas.present();
        Ok(())
    }

    pub fn textfields(&self) -> std::slice::Iter<TextField> {
        self.textfields.iter()
    }

    pub fn textfields_mut(&mut self) -> std::slice::IterMut<TextField> {
        self.textfields.iter_mut()
    }


    pub fn get_input(&self, idx: usize) -> String {
        if idx >= self.textfields.len() {
            panic!("get_input: Invalid textbox index")
        };
        self.textfields[idx].to_string()
    }

    pub fn set_textbox_content(&mut self, idx: usize, content: String) {
        if idx >= self.textfields.len() {
            return;
        };
        if let Some(textbox) = self.textfields.iter_mut().nth(idx) {
            textbox.set_content(content);
        }
    }

    pub fn push_to_textbox(&mut self, idx: usize, c: char) {
        self.textfields[idx].push(c.to_string());
    }

    pub fn pop_from_textbox(&mut self, idx: usize) -> Option<char> {
        self.textfields[idx].pop_char()
    }

    pub fn clear_textbox(&mut self, idx: usize) {
        self.textfields[idx].clear();
    }

    fn get_slider_value(&self, idx: usize) -> f32 {
        if idx >= self.sliders.len() {
            panic!("get_slider_value: Invalid slider index: {}, len: {}", idx, self.sliders.len())
        };
        self.sliders[idx].value()
    }

    fn switch_active_textbox(&mut self) {
        let first_clickable = self.textfields
            .iter()
            .position(|tb| tb.is_clickable());

        if first_clickable.is_none() {
            return
        }
        let first_clickable = first_clickable.unwrap();

        let active = self.textfields
            .iter()
            .position(|tb| tb.is_active());

        if active.is_none() {
            self.textfields[first_clickable].set_active(true);
            return
        }
        let active = active.unwrap();

        let mut next_clickable = self.textfields
            .iter()
            .enumerate()
            .skip(active+1)
            .find(|(_, tb)| tb.is_clickable())
            .map(|(idx, _)| idx);

        if next_clickable.is_none() {
            next_clickable = self.textfields
                .iter()
                .position(|tb| tb.is_clickable());
        }

        self.textfields[active].set_active(false);
        self.textfields[next_clickable.unwrap()].set_active(true);
    }

    fn get_bounds(&self) -> Vec<Rect> {
        let mut bounds = self
            .buttons
            .iter()
            .map(|button| button.bounds())
            .collect::<Vec<Rect>>();
        
        self.textfields
            .iter()
            .for_each(|textbox| bounds.push(textbox.rect()));

        self.sliders
            .iter()
            .for_each(|slider| bounds.push(slider.bounds()));
        
        assert_eq!(bounds.len(), self.buttons.len() + self.textfields.len() + self.sliders.len());
        bounds
    }

    fn which_widget(&self, idx: usize) -> (WidgetType, usize) {
        let buttons = self.buttons.len();
        let textfields = self.textfields.len();
        let sliders = self.sliders.len();
        if buttons + textfields + sliders == 0 {
            panic!("GUI::which_widget() called when program doesn't contain any")
        };
        if idx < buttons && buttons > 0 {
            (WidgetType::Button, idx)
        } else if idx - buttons < textfields {
            (WidgetType::TextField, idx - buttons)
        } else {
            (WidgetType::Slider, idx - buttons - textfields)
        }
    }

    fn deselect_textfields(&mut self) {
        self.textfields.iter_mut().for_each(|tb| {
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
    buttons: Vec<Button<T>>,
    textfields: Vec<TextField>,
    sliders: Vec<Slider>,
    font: &'static str,
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
            buttons: vec![],
            textfields: vec![],
            sliders: vec![],
            font: DEFAULTFONT,
        }
    }
    pub const fn color_rgb(mut self, r: u8, g: u8, b: u8) -> GuiBuilder<T> {
        self.backround_color = Color::RGB(r, g, b);
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
    pub fn sliders(mut self, sliders: Vec<Slider>) -> GuiBuilder<T> {
        self.sliders = sliders;
        self
    }
    pub const fn font(mut self, s: &'static str) -> GuiBuilder<T> {
        self.font = s;
        self
    }
    pub const fn size(mut self, w: u32, h: u32) -> GuiBuilder<T> {
        self.window_size.0 = w;
        self.window_size.1 = h;
        self
    }
    pub fn panel(mut self, panel: Panel<T>) -> GuiBuilder<T> {
        self.buttons = panel.buttons;
        self.textfields = panel.textfields;
        self.sliders = panel.sliders;
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
            textfields: self.textfields,
            sliders: self.sliders,
        });
    }
}