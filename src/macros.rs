use std::collections::BTreeSet;
use std::fmt;

use keys::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum MacroOutput {
    KeyPresses(Vec<KeyPress>),
    Shortcut(Shortcut),
}

impl MacroOutput {
    pub fn from_string(s: &str) -> MacroOutput {
        let mut key_presses = Vec::new();

        for c in s.chars() {
            let char_to_key = char_to_key(c);
            let requires_shift = requires_shift(c);
            let key_press = KeyPress::new(requires_shift, char_to_key);
            key_presses.push(key_press)
        }

        MacroOutput::KeyPresses(key_presses)
    }

    pub fn from_string_move_cursor(s: &str, back: u16) -> MacroOutput {
        let mut arrows = Vec::new();

        for _ in 0..back {
            arrows.push(KeyPress::not_shifted(NonModifier::LeftArrow));
        }

        let mut key_presses = Vec::new();

        for c in s.chars() {
            let char_to_key = char_to_key(c);
            let requires_shift = requires_shift(c);
            let key_press = KeyPress::new(requires_shift, char_to_key);
            key_presses.push(key_press)
        }

        key_presses.extend(arrows);

        MacroOutput::KeyPresses(key_presses)
    }

    pub fn shortcut_keypad_off(
        modifiers: BTreeSet<Modifier>,
        non_modifiers: NonModifier,
    ) -> MacroOutput {
        MacroOutput::Shortcut(Shortcut::keypad_off(modifiers, non_modifiers))
    }

    pub fn shortcut_keypad_on(
        modifiers: BTreeSet<Modifier>,
        non_modifiers: NonModifier,
    ) -> MacroOutput {
        MacroOutput::Shortcut(Shortcut::keypad_on(modifiers, non_modifiers))
    }
}

impl fmt::Display for MacroOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shifted = false;
        let mut string: String = String::new();

        match self {
            MacroOutput::KeyPresses(keys) => {
                for k in keys.iter() {
                    if !shifted && k.shifted {
                        shifted = true;
                        string.push_str(format!("{{-{}}}", Modifier::LeftShift).as_str());
                    }

                    if shifted && !k.shifted {
                        shifted = false;
                        string.push_str(format!("{{+{}}}", Modifier::LeftShift).as_str());
                    }

                    string.push_str(format!("{{{}}}", k.key).to_lowercase().as_str());
                }

                if shifted {
                    string.push_str(format!("{{+{}}}", Modifier::LeftShift).as_str());
                }
            }

            MacroOutput::Shortcut(shortcut) => {
                let keypad = &shortcut.keypad;
                string.push_str(
                    format!(
                        "{{{}}}",
                        KeyLayer::new(
                            keypad.clone(),
                            Key::NonModifier(shortcut.non_modifier.clone())
                        )
                    ).as_str(),
                );
                for key in shortcut.modifiers.iter() {
                    string.insert_str(0, format!("{{-{}}}", key).as_str());
                    string.push_str(format!("{{+{}}}", key).as_str());
                }
            }
        }

        write!(f, "{}", string)
    }
}

fn char_to_key(c: char) -> NonModifier {
    use self::NonModifier::*;

    match c {
        '=' | '+' => Equals,
        '1' | '!' => One,
        '2' | '@' => Two,
        '3' | '#' => Three,
        '4' | '$' => Four,
        '5' | '%' => Five,
        '6' | '^' => Six,
        '7' | '&' => Seven,
        '8' | '*' => Eight,
        '9' | '(' => Nine,
        '-' | '_' => Hyphen,
        'q' | 'Q' => Q,
        'w' | 'W' => W,
        'e' | 'E' => E,
        'r' | 'R' => R,
        't' | 'T' => T,
        'y' | 'Y' => Y,
        'u' | 'U' => U,
        'i' | 'I' => I,
        'o' | 'O' => O,
        'p' | 'P' => P,
        '\\' | '|' => BackSlash,
        '\t' => Tab,
        'a' | 'A' => A,
        's' | 'S' => S,
        'd' | 'D' => D,
        'f' | 'F' => F,
        'g' | 'G' => G,
        'h' | 'H' => H,
        'j' | 'J' => J,
        'k' | 'K' => K,
        'l' | 'L' => L,
        ';' | ':' => SemiColon,
        '\'' | '"' => Quote,
        'z' | 'Z' => Z,
        'x' | 'X' => X,
        'c' | 'C' => C,
        'v' | 'V' => V,
        'b' | 'B' => B,
        'n' | 'N' => N,
        'm' | 'M' => M,
        ',' | '<' => Comma,
        '.' | '>' => FullStop,
        '/' | '?' => ForwardSlash,
        '[' | '{' => OpenBracket,
        ']' | '}' => CloseBracket,
        '\n' => Enter,
        ' ' => Space,
        c => panic!("Oh No".to_owned() + format!("{}", c).as_str()),
    }
}

fn requires_shift(c: char) -> bool {
    let shifted_symbols = [
        '+', '!', '@', '#', '$', '%', '^', '&', '*', '(', '_', '|', '"', '<', '>', '?', '{', '}',
    ];

    c.is_ascii_uppercase() || shifted_symbols.contains(&c)
}
