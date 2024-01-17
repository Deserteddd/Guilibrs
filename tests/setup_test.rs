use guilibrs::*;

fn init() -> GUI<u32>{
    let gui: GUI<u32> = GuiBuilder::new()
    .size(600, 480)
    .build().unwrap();

    gui
}

#[test]
fn create_gui() {
    init();
}