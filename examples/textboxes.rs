extern crate guilibrs;

use guilibrs::gui::GUI;
use guilibrs::textfield::{TextField, TextAlign};
use guilibrs::button::Button;

fn main() -> Result<(), String> {
    let mut gui: GUI<u8> = GUI::new()
        .title("TextFields")
        .textfields(vec![
            TextField::new(20, 20, 340, 40)
                .label("Left aligned textfield")
                .align(TextAlign::Left)
                .clickable(),
            TextField::new(20, 80, 340, 40)
                .label("Center aligned textfield")
                .align(TextAlign::Center)
                .clickable(),
            TextField::new(20, 140, 340, 40)
                .label("Right aligned textfield")
                .align(TextAlign::Right)
                .clickable()
        ])
        .buttons(vec![
            Button::new(20, 200, 340, 40)
                .label("Print textfields")
                .color_rgb(255, 255, 255)
                .callback(1)
        ])
        .size(380, 540)
        .color_rgb(40, 40, 40)
        .build()?;

    'running: loop {
        match gui.poll() {
            guilibrs::gui::GuiEvent::None => {}
            guilibrs::gui::GuiEvent::Quit => break 'running,
            guilibrs::gui::GuiEvent::Callback(u) => match u {
                1 => gui
                .textfields()
                .filter(|t| !t.get_content().is_empty())
                .for_each(|t| println!("{}: {}", t.get_label(), t.get_content())),
                _ => {}
            }
        }
        gui.draw()?;
    }
    Ok(())
}