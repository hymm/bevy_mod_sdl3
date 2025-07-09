use bevy_ecs::world::World;
use bevy_input::{
    ButtonState,
    keyboard::{Key as BevyKey, KeyCode as BevyKeyCode, KeyboardInput, NativeKey, NativeKeyCode},
};
use sdl3::keyboard::{Keycode as SdlKeycode, Mod, Scancode as SdlScancode};

use crate::SDL_CONTEXT;

pub fn handle_keyboard_events(
    world: &mut World,
    button_state: ButtonState,
    _timestamp: u64,
    window_id: u32,
    keycode: Option<SdlKeycode>,
    scancode: Option<SdlScancode>,
    keymod: Mod,
    repeat: bool,
    _which: u32,
    _raw: u16,
) {
    let window = SDL_CONTEXT.with_borrow(|context| {
        context
            .as_ref()
            .unwrap()
            .windows
            .winit_to_entity
            .get(&window_id.into())
            .copied()
    });
    world.send_event(KeyboardInput {
        key_code: dbg!(convert_sdl_scancode_to_physical_key(
            scancode.unwrap_or(SdlScancode::Unknown)
        )),
        logical_key: convert_sdl_keycode_to_key(keycode.unwrap_or(SdlKeycode::Unknown), keymod),
        state: button_state,
        text: None,
        repeat,
        window: window.unwrap(),
    });
}

// disable formatting so we can place multiple match statements on the same line and reduce the height
#[rustfmt::skip]
fn convert_sdl_scancode_to_physical_key(scancode: SdlScancode) ->  BevyKeyCode {
    use BevyKeyCode::*;
    match scancode {
        SdlScancode::Unknown => Unidentified(NativeKeyCode::Unidentified),
        // alphas
        SdlScancode::A => KeyA, SdlScancode::B => KeyB, SdlScancode::C => KeyC, SdlScancode::D => KeyD, SdlScancode::E => KeyE,
        SdlScancode::F => KeyF, SdlScancode::G => KeyG, SdlScancode::H => KeyH, SdlScancode::I => KeyI, SdlScancode::J => KeyJ,
        SdlScancode::K => KeyK, SdlScancode::L => KeyL, SdlScancode::M => KeyM, SdlScancode::N => KeyN, SdlScancode::O => KeyO,
        SdlScancode::P => KeyP, SdlScancode::Q => KeyQ, SdlScancode::R => KeyR, SdlScancode::S => KeyS, SdlScancode::T => KeyT,
        SdlScancode::U => KeyU, SdlScancode::V => KeyV, SdlScancode::W => KeyW, SdlScancode::X => KeyX, SdlScancode::Y => KeyY,
        SdlScancode::Z => KeyZ,
        // numerals
        SdlScancode::_1 => Digit1, SdlScancode::_2 => Digit2, SdlScancode::_3 => Digit3, SdlScancode::_4 => Digit4,
        SdlScancode::_5 => Digit5, SdlScancode::_6 => Digit6, SdlScancode::_7 => Digit7, SdlScancode::_8 => Digit8,
        SdlScancode::_9 => Digit9, SdlScancode::_0 => Digit0,
        // whitespace
        SdlScancode::Return => Enter, SdlScancode::Escape => Escape, SdlScancode::Backspace => Backspace,
        SdlScancode::Tab => Tab, SdlScancode::Space => Space,
        // punctuation
        SdlScancode::Minus => Minus, SdlScancode::Equals => Equal, SdlScancode::LeftBracket => BracketLeft,
        SdlScancode::RightBracket => BracketRight, SdlScancode::Backslash => Backslash, SdlScancode::NonUsHash => todo!(),
        SdlScancode::Semicolon => Semicolon, SdlScancode::Apostrophe => Quote, SdlScancode::Grave => Backquote,
        SdlScancode::Comma => Comma, SdlScancode::Period => Period, SdlScancode::Slash => Slash,
        SdlScancode::CapsLock => CapsLock,
        // function keys
        SdlScancode::F1 => F1, SdlScancode::F2 => F2, SdlScancode::F3 => F3, SdlScancode::F4 => F4, SdlScancode::F5 => F5,
        SdlScancode::F6 => F6, SdlScancode::F7 => F7, SdlScancode::F8 => F8, SdlScancode::F9 => F9, SdlScancode::F10 => F10,
        SdlScancode::F11 => F11, SdlScancode::F12 => F12, SdlScancode::F13 => F13, SdlScancode::F14 => F14, SdlScancode::F15 => F15,
        SdlScancode::F16 => F16, SdlScancode::F17 => F17, SdlScancode::F18 => F18, SdlScancode::F19 => F19, SdlScancode::F20 => F20,
        SdlScancode::F21 => F21, SdlScancode::F22 => F22, SdlScancode::F23 => F23, SdlScancode::F24 => F24,
        // Navigation
        SdlScancode::PrintScreen => PrintScreen, SdlScancode::ScrollLock => ScrollLock, SdlScancode::Pause => Pause,
        SdlScancode::Insert => Insert, SdlScancode::Home => Home, SdlScancode::PageUp => PageUp,
        SdlScancode::Delete => Delete, SdlScancode::End => End, SdlScancode::PageDown => PageDown,
        SdlScancode::Right => ArrowRight, SdlScancode::Left => ArrowLeft, SdlScancode::Down => ArrowDown, SdlScancode::Up => ArrowUp,
        // Numpad
        SdlScancode::NumLockClear => NumLock, 
        SdlScancode::KpDivide => NumpadDivide, SdlScancode::KpMultiply => NumpadMultiply, SdlScancode::KpMinus => NumpadSubtract,
        SdlScancode::KpPlus => NumpadAdd, SdlScancode::KpEnter => NumpadEnter, SdlScancode::KpPeriod => NumpadDecimal,
        SdlScancode::Kp1 => Numpad1, SdlScancode::Kp2 => Numpad2, SdlScancode::Kp3 => Numpad3, SdlScancode::Kp4 => Numpad4,
        SdlScancode::Kp5 => Numpad5, SdlScancode::Kp6 => Numpad6, SdlScancode::Kp7 => Numpad7, SdlScancode::Kp8 => Numpad8,
        SdlScancode::Kp9 => Numpad9, SdlScancode::Kp0 => Numpad0,
        SdlScancode::KpComma => NumpadComma, SdlScancode::KpBackspace => NumpadBackspace, SdlScancode::KpMemStore => NumpadMemoryStore,
        SdlScancode::KpMemRecall => NumpadMemoryRecall, SdlScancode::KpMemClear => NumpadMemoryClear, SdlScancode::KpMemAdd => NumpadMemoryAdd,
        SdlScancode::KpMemSubtract => NumpadMemorySubtract, SdlScancode::KpEquals => NumpadEqual,
        // Modifiers
        SdlScancode::LCtrl => ControlLeft, SdlScancode::LShift => ShiftLeft, SdlScancode::LAlt => AltLeft, SdlScancode::LGui => SuperLeft, 
        SdlScancode::RCtrl => ControlRight, SdlScancode::RShift => ShiftRight, SdlScancode::RAlt => AltRight, SdlScancode::RGui => SuperRight,
        // Media
        SdlScancode::MediaNextTrack => MediaTrackNext, SdlScancode::MediaPreviousTrack => MediaTrackPrevious,
        SdlScancode::MediaStop => MediaStop, SdlScancode::MediaPlayPause => MediaPlayPause, SdlScancode::MediaSelect => MediaSelect,
        SdlScancode::Mute => AudioVolumeMute, SdlScancode::VolumeUp => AudioVolumeUp, SdlScancode::VolumeDown => AudioVolumeDown,
        // Browser
        SdlScancode::AcSearch => BrowserSearch, SdlScancode::AcHome => BrowserHome, SdlScancode::AcBack => BrowserBack,
        SdlScancode::AcForward => BrowserForward, SdlScancode::AcStop => BrowserStop, SdlScancode::AcRefresh => BrowserRefresh,
        SdlScancode::AcBookmarks => BrowserFavorites,
        // Other
        SdlScancode::NonUsBackslash => IntlBackslash,
        SdlScancode::Power => Power, SdlScancode::Help => Help, SdlScancode::Menu => ContextMenu, SdlScancode::Select => Select,
        SdlScancode::Again => Again, SdlScancode::Undo => Undo,
        SdlScancode::Cut => Cut, SdlScancode::Copy => Copy, SdlScancode::Paste => Paste, SdlScancode::Find => Find,
        SdlScancode::Lang1 => Lang1, SdlScancode::Lang2 => Lang2, SdlScancode::Lang3 => Lang3, 
        SdlScancode::Lang4 => Lang4, SdlScancode::Lang5 => Lang5,
        SdlScancode::Sleep => Sleep, SdlScancode::Wake => WakeUp,
        // Unsupported
        SdlScancode::Application | SdlScancode::Execute | SdlScancode::Stop | SdlScancode::KpEqualsAs400 |
            SdlScancode::International1 | SdlScancode::International2 | SdlScancode::International3 | SdlScancode::International4 |        
            SdlScancode::International5 | SdlScancode::International6 | SdlScancode::International7 | SdlScancode::International8 | 
            SdlScancode::International9 |
            SdlScancode::Lang6 | SdlScancode::Lang7 | SdlScancode::Lang8 | SdlScancode::Lang9 |
            SdlScancode::AltErase | SdlScancode::SysReq | SdlScancode::Cancel | SdlScancode::Clear |
            SdlScancode::Prior | SdlScancode::Return2 | SdlScancode::Separator | SdlScancode::Out | 
            SdlScancode::Oper | SdlScancode::ClearAgain | SdlScancode::CrSel | SdlScancode::ExSel | 
            SdlScancode::Kp00 | SdlScancode::Kp000 | 
            SdlScancode::ThousandsSeparator | SdlScancode::DecimalSeparator | SdlScancode::CurrencyUnit | SdlScancode::CurrencySubunit | 
            SdlScancode::KpLeftParen | SdlScancode::KpRightParen | SdlScancode::KpLeftBrace | SdlScancode::KpRightBrace | 
            SdlScancode::KpTab | 
            SdlScancode::KpA | SdlScancode::KpB | SdlScancode::KpC | SdlScancode::KpD | SdlScancode::KpE | SdlScancode::KpF | 
            SdlScancode::KpXor | SdlScancode::KpPower | SdlScancode::KpPercent | 
            SdlScancode::KpLess | SdlScancode::KpGreater | SdlScancode::KpAmpersand | SdlScancode::KpDblAmpersand | 
            SdlScancode::KpVerticalBar | SdlScancode::KpDblVerticalBar | SdlScancode::KpColon | SdlScancode::KpHash | 
            SdlScancode::KpSpace | SdlScancode::KpAt | SdlScancode::KpExclam | SdlScancode::KpMemMultiply | 
            SdlScancode::KpMemDivide | SdlScancode::KpPlusMinus | SdlScancode::KpClear | SdlScancode::KpClearEntry | 
            SdlScancode::KpBinary | SdlScancode::KpOctal | SdlScancode::KpDecimal | SdlScancode::KpHexadecimal | 
            SdlScancode::Mode | SdlScancode::ChannelIncrement | SdlScancode::ChannelDecrement | 
            SdlScancode::MediaPlay | SdlScancode::MediaPause | SdlScancode::MediaRecord | SdlScancode::MediaFastForward | 
            SdlScancode::MediaRewind | SdlScancode::MediaEject | 
            SdlScancode::AcNew | SdlScancode::AcOpen | SdlScancode::AcClose | SdlScancode::AcExit | 
            SdlScancode::AcSave | SdlScancode::AcPrint | SdlScancode::AcProperties | 
            SdlScancode::SoftLeft | SdlScancode::SoftRight | 
            SdlScancode::Call | SdlScancode::EndCall | SdlScancode::Reserved | 
            SdlScancode::Count => todo!(),
    }
}

// disable formatting so we can place multiple match statements on the same line and reduce the height
#[rustfmt::skip]
fn convert_sdl_keycode_to_key(keycode: SdlKeycode, modifier: Mod) -> BevyKey {
    fn char(s: &str) -> BevyKey {
        BevyKey::Character(s.into())
    }

    use bevy_input::keyboard::Key::*;
    match (keycode, modifier) {
        (SdlKeycode::Unknown, _) => Unidentified(NativeKey::Unidentified),
        (SdlKeycode::CapsLock, _) => CapsLock,
        // Whitespace
        (SdlKeycode::Return, _) => Enter, (SdlKeycode::Escape, _) => Escape, (SdlKeycode::Backspace, _) => Backspace,
        (SdlKeycode::Tab, _) => Tab, (SdlKeycode::Space, _) => Space,
        // Punctuation
        (SdlKeycode::Apostrophe, _) => char("'"), (SdlKeycode::Comma, _) => char(","), (SdlKeycode::Minus, _) => char("-"),
        (SdlKeycode::Period, _) => char("."), (SdlKeycode::Slash, _) => char("/"), (SdlKeycode::Exclaim, _) => char("!"),
        (SdlKeycode::DblApostrophe, _) => char("\""), (SdlKeycode::Hash, _) => char("#"), (SdlKeycode::Dollar, _) => char("$"),
        (SdlKeycode::Percent, _) => char("%"), (SdlKeycode::Ampersand, _) => char("&"), (SdlKeycode::LeftParen, _) => char("("),
        (SdlKeycode::RightParen, _) => char(")"), (SdlKeycode::Asterisk, _) => char("*"), (SdlKeycode::Plus, _) => char("+"),
        (SdlKeycode::Semicolon, _) => char(";"), (SdlKeycode::Equals, _) => char("="), (SdlKeycode::Backslash, _) => char("\\"),
        (SdlKeycode::RightBracket, _) => char("]"), (SdlKeycode::Colon, _) => char(":"), (SdlKeycode::Less, _) => char("<"),
        (SdlKeycode::Greater, _) => char(">"), (SdlKeycode::Question, _) => char("?"), (SdlKeycode::At, _) => char("@"),
        (SdlKeycode::LeftBracket, _) => char("["), (SdlKeycode::Caret, _) => char("^"), (SdlKeycode::Underscore, _) => char("_"),
        (SdlKeycode::Grave, _) => char("`"), (SdlKeycode::LeftBrace, _) => char("{"), (SdlKeycode::Pipe, _) => char("|"),
        (SdlKeycode::RightBrace, _) => char("}"), (SdlKeycode::Tilde, _) => char("~"), (SdlKeycode::PlusMinus, _) => char("±"),
        // numbers
        (SdlKeycode::_0, _) => char("0"), (SdlKeycode::_1, _) => char("1"), (SdlKeycode::_2, _) => char("2"), 
        (SdlKeycode::_3, _) => char("3"), (SdlKeycode::_4, _) => char("4"), (SdlKeycode::_5, _) => char("5"), (SdlKeycode::_6, _) => char("6"),
        (SdlKeycode::_7, _) => char("7"), (SdlKeycode::_8, _) => char("8"), (SdlKeycode::_9, _) => char("9"),
        // alphas
        (SdlKeycode::A, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("A"), (SdlKeycode::A, _) => char("a"),
        (SdlKeycode::B, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("B"), (SdlKeycode::B, _) => char("b"),
        (SdlKeycode::C, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("C"), (SdlKeycode::C, _) => char("c"),
        (SdlKeycode::D, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("D"), (SdlKeycode::D, _) => char("d"),
        (SdlKeycode::E, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("E"), (SdlKeycode::E, _) => char("e"),
        (SdlKeycode::F, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("F"), (SdlKeycode::F, _) => char("f"),
        (SdlKeycode::G, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("G"), (SdlKeycode::G, _) => char("g"),
        (SdlKeycode::H, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("H"), (SdlKeycode::H, _) => char("h"),
        (SdlKeycode::I, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("I"), (SdlKeycode::I, _) => char("i"),
        (SdlKeycode::J, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("J"), (SdlKeycode::J, _) => char("j"),
        (SdlKeycode::K, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("K"), (SdlKeycode::K, _) => char("k"),
        (SdlKeycode::L, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("L"), (SdlKeycode::L, _) => char("l"),
        (SdlKeycode::M, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("M"), (SdlKeycode::M, _) => char("m"),
        (SdlKeycode::N, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("N"), (SdlKeycode::N, _) => char("n"),
        (SdlKeycode::O, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("O"), (SdlKeycode::O, _) => char("o"),
        (SdlKeycode::P, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("P"), (SdlKeycode::P, _) => char("p"),
        (SdlKeycode::Q, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("Q"), (SdlKeycode::Q, _) => char("q"),
        (SdlKeycode::R, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("R"), (SdlKeycode::R, _) => char("r"),
        (SdlKeycode::S, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("S"), (SdlKeycode::S, _) => char("s"),
        (SdlKeycode::T, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("T"), (SdlKeycode::T, _) => char("t"),
        (SdlKeycode::U, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("U"), (SdlKeycode::U, _) => char("u"),
        (SdlKeycode::V, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("V"), (SdlKeycode::V, _) => char("v"),
        (SdlKeycode::W, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("W"), (SdlKeycode::W, _) => char("w"),
        (SdlKeycode::X, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("X"), (SdlKeycode::X, _) => char("x"),
        (SdlKeycode::Y, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("Y"), (SdlKeycode::Y, _) => char("y"),
        (SdlKeycode::Z, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => char("Z"), (SdlKeycode::Z, _) => char("z"),
        // function keys
        (SdlKeycode::F1, _) => F1, (SdlKeycode::F2, _) => F2, (SdlKeycode::F3, _) => F3, (SdlKeycode::F4, _) => F4,
        (SdlKeycode::F5, _) => F5, (SdlKeycode::F6, _) => F6, (SdlKeycode::F7, _) => F7, (SdlKeycode::F8, _) => F8,
        (SdlKeycode::F9, _) => F9, (SdlKeycode::F10, _) => F10, (SdlKeycode::F11, _) => F11, (SdlKeycode::F12, _) => F12,
        (SdlKeycode::F13, _) => F13, (SdlKeycode::F14, _) => F14, (SdlKeycode::F15, _) => F15, (SdlKeycode::F16, _) => F16,
        (SdlKeycode::F17, _) => F17, (SdlKeycode::F18, _) => F18, (SdlKeycode::F19, _) => F19, (SdlKeycode::F20, _) => F20,
        (SdlKeycode::F21, _) => F21, (SdlKeycode::F22, _) => F22, (SdlKeycode::F23, _) => F23, (SdlKeycode::F24, _) => F24,
        // Navigation
        (SdlKeycode::PrintScreen, _) => PrintScreen, (SdlKeycode::ScrollLock, _) => ScrollLock, (SdlKeycode::Pause, _) => Pause,
        (SdlKeycode::Insert, _) => Insert, (SdlKeycode::Home, _) => Home, (SdlKeycode::PageUp, _) => PageUp,
        (SdlKeycode::Delete, _) => Delete, (SdlKeycode::End, _) => End, (SdlKeycode::PageDown, _) => PageDown,
        (SdlKeycode::Up, _) => ArrowUp, (SdlKeycode::Left, _) => ArrowLeft, 
        (SdlKeycode::Down, _) => ArrowDown, (SdlKeycode::Right, _) => ArrowRight,
        // Numpad
        (SdlKeycode::NumLockClear, _) => NumLock, // not sure about this
        (SdlKeycode::KpDivide, _) => char("/"), (SdlKeycode::KpMultiply, _) => char("*"), 
        (SdlKeycode::KpMinus, _) => char("-"), (SdlKeycode::KpPlus, _) => char("+"), 
        (SdlKeycode::KpEnter, _) => Enter,
        (SdlKeycode::Kp0, _) => char("0"), (SdlKeycode::Kp1, _) => char("1"), (SdlKeycode::Kp2, _) => char("2"), 
        (SdlKeycode::Kp3, _) => char("3"), (SdlKeycode::Kp4, _) => char("4"), (SdlKeycode::Kp5, _) => char("5"),
        (SdlKeycode::Kp6, _) => char("6"), (SdlKeycode::Kp7, _) => char("7"), (SdlKeycode::Kp8, _) => char("8"),
        (SdlKeycode::Kp9, _) => char("9"),
        (SdlKeycode::KpPeriod, _) => char("."), (SdlKeycode::KpEquals, _) => char("="),
        (SdlKeycode::KpLeftParen, _) => char("("), (SdlKeycode::KpRightParen, _) => char(")"),
        // Other
        (SdlKeycode::Application, _) => AppSwitch, (SdlKeycode::Power, _) => Power, (SdlKeycode::Execute, _) => Execute,
        (SdlKeycode::Help, _) => Help, (SdlKeycode::Menu, _) => ContextMenu, (SdlKeycode::Select, _) => Select,
        (SdlKeycode::Again, _) => Again, (SdlKeycode::Undo, _) => Undo, (SdlKeycode::Cut, _) => Cut,
        (SdlKeycode::Copy, _) => Copy, (SdlKeycode::Paste, _) => Paste, (SdlKeycode::Find, _) => Find,
        // Audio
        (SdlKeycode::Mute, _) => AudioVolumeMute, (SdlKeycode::VolumeUp, _) => AudioVolumeUp, (SdlKeycode::VolumeDown, _) => AudioVolumeDown,
        (SdlKeycode::Cancel, _) => Cancel, (SdlKeycode::Clear, _) => Clear,
        (SdlKeycode::CrSel, _) => CrSel, (SdlKeycode::ExSel, _) => ExSel,
        // Modifiers
        (SdlKeycode::LCtrl | SdlKeycode::RCtrl, _) => Control, (SdlKeycode::LShift | SdlKeycode::RShift, _) => Shift,
        (SdlKeycode::LAlt | SdlKeycode::RAlt, _) => Alt, (SdlKeycode::LGui | SdlKeycode::RGui, _) => Super,
        // Media
        (SdlKeycode::MediaSelect, _) => MediaTopMenu, (SdlKeycode::MediaNextTrack, _) => MediaTrackNext,
        (SdlKeycode::MediaPreviousTrack, _) => MediaTrackPrevious, (SdlKeycode::MediaStop, _) => MediaStop,
        (SdlKeycode::MediaPlayPause, _) => MediaPlayPause, (SdlKeycode::MediaPlay, _) => MediaPlay,
        (SdlKeycode::MediaPause, _) => MediaPause, (SdlKeycode::MediaRecord, _) => MediaRecord,
        (SdlKeycode::MediaFastForward, _) => MediaFastForward, (SdlKeycode::MediaRewind, _) => MediaRewind,
        // Power
        (SdlKeycode::Wake, _) => WakeUp,
        // Browser
        (SdlKeycode::AcSearch, _) => BrowserSearch, (SdlKeycode::AcHome, _) => BrowserHome,
        (SdlKeycode::AcBack, _) => BrowserBack, (SdlKeycode::AcForward, _) => BrowserForward,
        (SdlKeycode::AcStop, _) => BrowserStop, (SdlKeycode::AcRefresh, _) => BrowserRefresh,
        (SdlKeycode::AcBookmarks, _) => BrowserFavorites,
        // Unsupported
        (SdlKeycode::Sleep | SdlKeycode::KpMemStore | SdlKeycode::KpMemRecall | SdlKeycode::KpMemClear |
            SdlKeycode::KpMemAdd | SdlKeycode::KpMemSubtract | SdlKeycode::KpClear | SdlKeycode::KpClearEntry |
            SdlKeycode::Kp00 | SdlKeycode::Kp000 | SdlKeycode::KpLeftBrace | SdlKeycode::KpRightBrace | 
            SdlKeycode::KpTab | SdlKeycode::KpBackspace | SdlKeycode::KpA | SdlKeycode::KpB | SdlKeycode::KpC |
            SdlKeycode::KpD | SdlKeycode::KpE | SdlKeycode::KpF | SdlKeycode::KpXor | SdlKeycode::KpPower | 
            SdlKeycode::KpPercent | SdlKeycode::KpLess | SdlKeycode::KpGreater | SdlKeycode::KpAmpersand |
            SdlKeycode::KpDblAmpersand | SdlKeycode::KpVerticalBar | SdlKeycode::KpDblVerticalBar | SdlKeycode::KpExclam|
            SdlKeycode::KpColon | SdlKeycode::KpHash | SdlKeycode::KpSpace | SdlKeycode::KpAt |
            SdlKeycode::KpMemMultiply | SdlKeycode::KpMemDivide | SdlKeycode::KpPlusMinus | SdlKeycode::KpBinary |
            SdlKeycode::KpOctal | SdlKeycode::KpDecimal | SdlKeycode::KpHexadecimal | SdlKeycode::KpComma |
            SdlKeycode::KpEqualsAs400 | SdlKeycode::ScancodeMask | SdlKeycode::AltErase | SdlKeycode::SysReq |
            SdlKeycode::Prior | SdlKeycode::Return2 | SdlKeycode::Separator | SdlKeycode::Out | SdlKeycode::Oper |
            SdlKeycode::ClearAgain | SdlKeycode::ThousandsSeparator | SdlKeycode::DecimalSeparator |
            SdlKeycode::CurrencyUnit | SdlKeycode::CurrencySubunit | SdlKeycode::Stop | SdlKeycode::Mode |
            SdlKeycode::ChannelIncrement | SdlKeycode::ChannelDecrement | SdlKeycode::MediaEject |
            SdlKeycode::AcNew | SdlKeycode::AcOpen | SdlKeycode::AcClose | SdlKeycode::AcExit | SdlKeycode::AcSave |
            SdlKeycode::AcPrint | SdlKeycode::AcProperties | SdlKeycode::SoftLeft | SdlKeycode::SoftRight |
            SdlKeycode::Call | SdlKeycode::EndCall, _) => todo!(),
    }
}
