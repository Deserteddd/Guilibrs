use sdl2::event::Event;
use sdl2::keyboard::{KeyboardUtil, Keycode, Mod, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::{EventPump, Sdl};

pub struct EventHandler {
    pump: EventPump,
    kb_parser: KBParser,
    hovered: Option<usize>,
    lmb_down: (bool, i32, i32), // (is_down, x, y)
}

impl EventHandler {
    pub fn new(context: &Sdl) -> Result<EventHandler, String> {
        Ok(EventHandler {
            pump: context.event_pump()?,
            kb_parser: KBParser::new(&context),
            hovered: None,
            lmb_down: (false, 0, 0),
        })
    }
    pub fn poll(&mut self, bounds: &[Rect]) -> HandlerEvent {
        match self.pump.wait_event() {
            Event::Quit { .. } => HandlerEvent::Quit,
            Event::KeyDown { scancode, keycode, .. } => {
                match self.kb_parser.parse_keycode(keycode, scancode) {
                    ParsedKey::Event(handler_ins) => handler_ins,
                    ParsedKey::Ignore => HandlerEvent::None,
                }
            },
            Event::MouseMotion { x, y, .. } => {
                let mut hover_instruction: Option<HandlerEvent> = None;
                if let Some(idx) = self.hovered {
                    if !in_bounds(&bounds[idx], x, y) {
                        hover_instruction = Some(HandlerEvent::UnHover(idx))
                    }
                } else {
                    for (idx, b) in bounds.iter().enumerate() {
                        if in_bounds(b, x, y) {
                            hover_instruction = Some(HandlerEvent::Hover(idx));
                        }
                    }
                }
                match hover_instruction {
                    Some(HandlerEvent::Hover(idx)) => {
                        self.hovered = Some(idx);
                        HandlerEvent::Hover(idx)
                    }
                    Some(HandlerEvent::UnHover(idx)) => {
                        self.hovered = None;
                        HandlerEvent::UnHover(idx)
                    }
                    Some(_) => HandlerEvent::None,
                    None => HandlerEvent::None,
                }
            },
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn == MouseButton::Left {
                    self.lmb_down = (true, x, y);
                }
                HandlerEvent::None
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                if mouse_btn == MouseButton::Left {
                    self.lmb_down.0 = false;
                    if let Some(idx) = self.hovered {
                        return HandlerEvent::Click(idx)
                    } else {
                        return HandlerEvent::ClickBackround
                    }
                }
                HandlerEvent::None
            }
            _ => HandlerEvent::None,
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ParsedKey {
    Event(HandlerEvent),
    Ignore,
}

struct KBParser {
    keyboard: KeyboardUtil,
}
impl KBParser {
    pub fn new(context: &Sdl) -> KBParser {
        KBParser {
            keyboard: context.keyboard(),
        }
    }

    pub fn parse_keycode(&self, kc: Option<Keycode>, sc: Option<Scancode>) -> ParsedKey {
        use ParsedKey::*;

        let shift = self.keyboard.mod_state().contains(Mod::LSHIFTMOD)
            || self.keyboard.mod_state().contains(Mod::RSHIFTMOD);
        let ctrl = self.keyboard.mod_state().contains(Mod::LCTRLMOD)
            || self.keyboard.mod_state().contains(Mod::RCTRLMOD);
        let altgr = (self.keyboard.mod_state().contains(Mod::LALTMOD)
            || self.keyboard.mod_state().contains(Mod::RALTMOD))
            && ctrl;

        if let Some(keycode) = kc {
            match keycode as u8 {
                8 => return Event(HandlerEvent::PopChar),
                9 => return Event(HandlerEvent::TabPress),
                13 => return Event(HandlerEvent::Return),
                27 => return Event(HandlerEvent::Escape),
                32 => {
                    return match altgr {
                        true => Ignore,
                        false => Event(HandlerEvent::PushChar(' ' as u8)),
                    }
                }
                39 => {
                    return match shift {
                        true => Event(HandlerEvent::PushChar('*' as u8)),
                        false => Event(HandlerEvent::PushChar(39)),
                    }
                }
                43 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('?' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('\\' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar(43)),
                        _ => Ignore,
                    }
                }
                44 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar(';' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar(',' as u8)),
                        _ => Ignore,
                    }
                }
                45 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('_' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('-' as u8)),
                        _ => Ignore,
                    }
                }
                46 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar(':' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('.' as u8)),
                        _ => Ignore,
                    }
                }
                48 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('=' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('}' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('0' as u8)),
                        _ => Ignore,
                    }
                }
                49 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('!' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('1' as u8)),
                        _ => Ignore,
                    }
                }
                50 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('"' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('@' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('2' as u8)),
                        _ => Ignore,
                    }
                }
                51 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('#' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('£' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('3' as u8)),
                        _ => Ignore,
                    }
                }
                52 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('¤' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('$' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('4' as u8)),
                        _ => Ignore,
                    }
                }
                53 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('%' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('€' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('5' as u8)),
                        _ => Ignore,
                    }
                }
                54 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('&' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('6' as u8)),
                        _ => Ignore,
                    }
                }
                55 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('/' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('{' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('7' as u8)),
                        _ => Ignore,
                    }
                }
                56 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('(' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('[' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('8' as u8)),
                        _ => Ignore,
                    }
                }
                57 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar(')' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar(']' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('9' as u8)),
                        _ => Ignore,
                    }
                }

                60 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('>' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('|' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('<' as u8)),
                        _ => Ignore,
                    }
                }
                96..=122 => {
                    return match self.should_capitalize() {
                        true => Event(HandlerEvent::PushChar(keycode as u8 - 32)),
                        false => Event(HandlerEvent::PushChar(keycode as u8)),
                    }
                }
                _ => {}
            }
        } else if let Some(scancode) = sc {
            return match scancode as u8 {
                46 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('`' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('´' as u8)),
                        _ => Ignore,
                    }
                }
                47 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('Å' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('å' as u8)),
                        _ => Ignore,
                    }
                }
                48 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('^' as u8)),
                        (false, true) => Event(HandlerEvent::PushChar('~' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar(34)),
                        _ => Ignore,
                    }
                }
                51 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('Ö' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('ö' as u8)),
                        _ => Ignore,
                    }
                }
                52 => {
                    return match (shift, altgr) {
                        (true, false) => Event(HandlerEvent::PushChar('Ä' as u8)),
                        (false, false) => Event(HandlerEvent::PushChar('ä' as u8)),
                        _ => Ignore,
                    }
                }
                _ => Ignore,
            };
        }

        Ignore
    }

    fn should_capitalize(&self) -> bool {
        let state = self.keyboard.mod_state();
        match state.contains(Mod::CAPSMOD) {
            true => {
                if state.contains(Mod::LSHIFTMOD) | state.contains(Mod::RSHIFTMOD) {
                    false
                } else {
                    true
                }
            }
            false => {
                if state.contains(Mod::LSHIFTMOD) | state.contains(Mod::RSHIFTMOD) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandlerEvent {
    Quit,
    Hover(usize),
    UnHover(usize),
    Click(usize),
    Escape,
    Return,
    PushChar(u8),
    PopChar,
    ClickBackround,
    TabPress,
    None
}

fn in_bounds(rect: &Rect, x: i32, y: i32) -> bool {
    if x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h {
        return true;
    }
    false
}
