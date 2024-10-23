extern crate guilibrs;

use guilibrs::gui::GUI;
use guilibrs::widget::{Textbox, TextAlign};

fn main() -> Result<(), String> {
    let mut gui: GUI<()> = GUI::new()
        .title("Textboxes")
        .textboxes(vec![
            Textbox::new(20, 20, 340, 40)
                .label("username")
                .align(TextAlign::Center)
                .clickable(),
            Textbox::new(20, 80, 340, 40)
                .label("email")
                .align(TextAlign::Center)
                .clickable(),
            Textbox::new(20, 140, 340, 40)
                .label("password")
                .align(TextAlign::Center)
                .clickable()
        ])
        .size(380, 540)
        .color_rgb(40, 40, 40)
        .build()?;

    'running: loop {
        match gui.poll() {
            guilibrs::gui::GuiEvent::None => {}
            guilibrs::gui::GuiEvent::Quit => break 'running,
            _ => {}
        }
        gui.draw()?;
    }
    Ok(())
}