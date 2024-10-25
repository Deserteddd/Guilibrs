extern crate guilibrs;

use guilibrs::gui::{Panel, GUI};
use guilibrs::slider::Slider;
use guilibrs::textfield::TextField;
use guilibrs::button::Button;

fn main() -> Result<(), String> {
    let login_screen = Panel::new(
        vec![
            Button::new(20, 480, 340, 40)
                .label("Login")
                .color_rgb(20, 120, 20)
                .callback(1)
        ],
        vec![
            TextField::new(20, 20, 340, 40)
                .label("Username")
                .clickable(),
            TextField::new(20, 80, 340, 40)
                .label("Password")
                .password()
                .clickable()
        ],
        vec![
            Slider::new(20, 140, 340)
        ]
    );

    let mut main_screen = Panel::new(
        vec![
            Button::new(20, 480, 340, 40)
                .label("Logout")
                .color_rgb(120, 20, 20)
                .callback(1),
            Button::new(20, 420, 340, 40)
                .label("Do stuff")
                .font_size(32)
                .color_rgb(0, 180, 180)
                .callback(2)
        ],
        vec![],
        vec![]
    );

    let mut gui: GUI<u8> = GUI::new()
        .panel(login_screen)
        .title("TextFields")
        .size(380, 540)
        .color_rgb(40, 40, 40)
        .build()?;

    'running: loop {
        match gui.poll() {
            guilibrs::gui::GuiEvent::None => {}
            guilibrs::gui::GuiEvent::Quit => break 'running,
            guilibrs::gui::GuiEvent::Callback(u) => match u {
                1 => { 
                    gui.textfields().for_each(|tf| println!("{}", tf));
                    main_screen = gui.swap_panel(main_screen); 
                },
                2 => println!("Doing stuff..."),
                _ => {}
            }
            guilibrs::gui::GuiEvent::NewSliderValue(u, f) => {
                println!("Slider {} moved to {}", u, f);
            }
        }
        gui.draw()?;
    }
    Ok(())
}