use std::collections::BTreeSet;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Keypad {
    Off,
    On,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Modifier {
    LeftShift,
    RightShift,
    LeftWindowsCommand,
    RightWindowsCommand,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Modifier::*;

        match self {
            LeftShift => write!(f, "lshift"),
            RightShift => write!(f, "rshift"),
            LeftWindowsCommand => write!(f, "lwin"),
            RightWindowsCommand => write!(f, "rwin"),
            LeftControl => write!(f, "lctrl"),
            RightControl => write!(f, "rctrl"),
            LeftAlt => write!(f, "lalt"),
            RightAlt => write!(f, "ralt"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum NonModifier {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Backtick,
    Hyphen,
    Equals,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    BackSlash,
    SemiColon,
    Quote,
    Comma,
    FullStop,
    ForwardSlash,
    OpenBracket,
    CloseBracket,
    Enter,
    PageUp,
    Tab,
    PageDown,
    Space,
    LeftArrow,
    Delete,
    RightArrow,
    Backspace,
    UpArrow,
    Insert,
    DownArrow,
    Home,
    End,
}

impl fmt::Display for NonModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::keys::NonModifier::*;

        match self {
            One => write!(f, "1"),
            Two => write!(f, "2"),
            Three => write!(f, "3"),
            Four => write!(f, "4"),
            Five => write!(f, "5"),
            Six => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine => write!(f, "9"),
            Zero => write!(f, "0"),
            Backtick => write!(f, "`"),
            Hyphen => write!(f, "hyphen"),
            Equals => write!(f, "="),
            BackSlash => write!(f, "\\"),
            SemiColon => write!(f, ";"),
            Quote => write!(f, "\'"),
            Comma => write!(f, ","),
            FullStop => write!(f, "."),
            ForwardSlash => write!(f, "/"),
            OpenBracket => write!(f, "obrack"),
            CloseBracket => write!(f, "cbrack"),
            Enter => write!(f, "enter"),
            PageUp => write!(f, "pup"),
            Tab => write!(f, "tab"),
            PageDown => write!(f, "pdown"),
            Space => write!(f, "space"),
            LeftArrow => write!(f, "left"),
            Delete => write!(f, "delete"),
            RightArrow => write!(f, "right"),
            Backspace => write!(f, "bspace"),
            UpArrow => write!(f, "up"),
            Insert => write!(f, "insert"),
            DownArrow => write!(f, "down"),
            Home => write!(f, "home"),
            End => write!(f, "end"),
            key => write!(f, "{}", format!("{:?}", key)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Key {
    Modifier(Modifier),
    NonModifier(NonModifier),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::Modifier(key) => write!(f, "{}", key),
            Key::NonModifier(key) => write!(f, "{}", key),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct KeyLayer {
    keypad_state: Keypad,
    key: Key,
}

impl KeyLayer {
    pub fn new(keypad_state: Keypad, key: Key) -> Self {
        KeyLayer { keypad_state, key }
    }

    pub fn off(key: Key) -> Self {
        KeyLayer::new(Keypad::Off, key)
    }

    pub fn on(key: Key) -> Self {
        KeyLayer::new(Keypad::On, key)
    }
}

impl fmt::Display for KeyLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keypad_state == Keypad::Off {
            write!(f, "{}", self.key)
        } else {
            match self.key {
                Key::NonModifier(NonModifier::Space) => write!(f, "kp0"),
                Key::NonModifier(NonModifier::M) => write!(f, "kp1"),
                Key::NonModifier(NonModifier::Comma) => write!(f, "kp2"),
                Key::NonModifier(NonModifier::FullStop) => write!(f, "kp3"),
                Key::NonModifier(NonModifier::J) => write!(f, "kp4"),
                Key::NonModifier(NonModifier::K) => write!(f, "kp5"),
                Key::NonModifier(NonModifier::L) => write!(f, "kp6"),
                Key::NonModifier(NonModifier::U) => write!(f, "kp7"),
                Key::NonModifier(NonModifier::I) => write!(f, "kp8"),
                Key::NonModifier(NonModifier::O) => write!(f, "kp9"),
                Key::NonModifier(NonModifier::Seven) => write!(f, "numlk"),
                Key::NonModifier(NonModifier::CloseBracket) => write!(f, "k."),
                Key::NonModifier(NonModifier::Eight) => write!(f, "k="),
                Key::NonModifier(NonModifier::Nine) => write!(f, "kpdiv"),
                Key::NonModifier(NonModifier::SemiColon) => write!(f, "kpplus"),
                Key::NonModifier(NonModifier::Zero) => write!(f, "kpmult"),
                Key::NonModifier(NonModifier::P) => write!(f, "kpmin"),
                Key::NonModifier(NonModifier::ForwardSlash) => write!(f, "kpenter1"),
                Key::NonModifier(ref key) => write!(f, "kp-{}", key),
                Key::Modifier(ref key) => write!(f, "kp-{}", key),
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct KeyPress {
    pub shifted: bool,
    pub key: NonModifier,
}

impl KeyPress {
    pub fn new(shifted: bool, key: NonModifier) -> Self {
        KeyPress { shifted, key }
    }

    pub fn not_shifted(key: NonModifier) -> Self {
        KeyPress::new(false, key)
    }

    pub fn shifted(key: NonModifier) -> Self {
        KeyPress::new(true, key)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Shortcut {
    pub keypad: Keypad,
    pub modifiers: BTreeSet<Modifier>,
    pub non_modifier: NonModifier,
}

impl Shortcut {
    fn new(keypad: Keypad, modifiers: BTreeSet<Modifier>, non_modifier: NonModifier) -> Shortcut {
        Shortcut {
            keypad,
            modifiers,
            non_modifier,
        }
    }

    pub fn keypad_off(modifiers: BTreeSet<Modifier>, non_modifier: NonModifier) -> Shortcut {
        Shortcut::new(Keypad::Off, modifiers, non_modifier)
    }

    pub fn keypad_on(modifiers: BTreeSet<Modifier>, non_modifier: NonModifier) -> Shortcut {
        Shortcut::new(Keypad::On, modifiers, non_modifier)
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = String::new();

        for m in self.modifiers.iter() {
            string.push_str(
                format!("{{{}}}", KeyLayer::new(self.keypad, Key::Modifier(*m))).as_str(),
            );
        }

        string.push_str(
            format!(
                "{{{}}}",
                KeyLayer::new(self.keypad, Key::NonModifier(self.non_modifier))
            ).as_str(),
        );

        write!(f, "{}", string)
    }
}
