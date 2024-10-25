use crate::Render;
use sdl2::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slider {
    position: (i32, i32),
    width: i32,
    value: f32,
}

impl Slider {
    pub fn new(x: i32, y: i32, w: i32) -> Slider {
        Slider {
            position: (x, y),
            width: w,
            value: 0.5,
        }
    }
    pub const fn value(&self) -> f32 {
        self.value
    }
    pub fn bounds(&self) -> Rect {
        let lerp = (self.value * self.width as f32) as i32;
        Rect::new(
                self.position.0 + lerp - 5,
                self.position.1 - 10,
                10,
                20
        )
    }
    pub fn drag(&mut self, x: i32) {
        let x = x - self.position.0;
        self.value = x as f32 / self.width as f32;
        if self.value < 0.0 {
            self.value = 0.0;
        } else if self.value > 1.0 {
            self.value = 1.0;
        }
    }
}

impl Render for Slider {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let lerp = (self.value * self.width as f32) as i32;
        for i in 0..3 {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
            canvas.draw_line(
                (self.position.0, self.position.1 + i),
                (self.position.0 + self.width, self.position.1 + i)
            )?;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
            canvas.draw_line(
                (self.position.0, self.position.1 + i),
                (self.position.0 + lerp, self.position.1 + i)
            )?;
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
        canvas.fill_rect(Rect::new(
            self.position.0 + lerp - 5,
            self.position.1 - 10,
            10,
            20
        ))?;        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 225, 150));
        canvas.draw_rect(Rect::new(
            self.position.0 + lerp - 5,
            self.position.1 - 10,
            10,
            20
        ))?;

        Ok(())
    }
}