use std::fmt;

use either::*;

use keys::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct MacroOutput(Vec<Either<Vec<KeyPress>, Shortcut>>);

impl MacroOutput {
    pub fn from_string(s: &str) -> MacroOutput {
        MacroOutput(vec![Left(string_to_key_presses(s))])
    }

    pub fn from_string_move_cursor(s: &str, back: usize) -> MacroOutput {
        let mut key_presses = string_to_key_presses(s);

        let arrows = vec![KeyPress::not_shifted(NonModifier::LeftArrow); back];
        key_presses.extend(arrows);

        MacroOutput(vec![Left(key_presses)])
    }

    pub fn shortcut(shortcut: Shortcut) -> MacroOutput {
        MacroOutput(vec![Right(shortcut)])
    }
}

impl fmt::Display for MacroOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = String::new();

        for out in self.0.iter() {
            match out {
                Left(keys) => {
                    let mut shifted = false;

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

                Right(shortcut) => {
                    let keypad = &shortcut.keypad;
                    string.push_str(
                        format!(
                            "{{{}}}",
                            KeyLayer::new(
                                *keypad,
                                Key::NonModifier(shortcut.non_modifier)
                            )
                        ).as_str(),
                    );
                    for key in shortcut.modifiers.iter() {
                        string.insert_str(0, format!("{{-{}}}", key).as_str());
                        string.push_str(format!("{{+{}}}", key).as_str());
                    }
                }
            }
        }

        write!(f, "{}", string)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug, Default)]
pub struct MacroBuilder(Vec<Either<Vec<KeyPress>, Shortcut>>);

impl MacroBuilder {
    pub fn new() -> MacroBuilder {
        Default::default()
    }

    pub fn with_string(&mut self, s: &str) -> &mut MacroBuilder {
        self.0.push(Left(string_to_key_presses(s)));
        self
    }

    pub fn with_shortcut(&mut self, shortcut: Shortcut) -> &mut MacroBuilder {
        self.0.push(Right(shortcut));
        self
    }

    pub fn cursor_up(&mut self, up: usize) -> &mut MacroBuilder {
        self.0
            .push(Left(vec![KeyPress::not_shifted(NonModifier::UpArrow); up]));
        self
    }

    pub fn cursor_down(&mut self, down: usize) -> &mut MacroBuilder {
        self.0.push(Left(vec![
            KeyPress::not_shifted(NonModifier::DownArrow);
            down
        ]));
        self
    }

    pub fn cursor_left(&mut self, left: usize) -> &mut MacroBuilder {
        self.0.push(Left(vec![
            KeyPress::not_shifted(NonModifier::LeftArrow);
            left
        ]));
        self
    }

    pub fn cursor_right(&mut self, right: usize) -> &mut MacroBuilder {
        self.0.push(Left(vec![
            KeyPress::not_shifted(NonModifier::RightArrow);
            right
        ]));
        self
    }

    pub fn make(&self) -> MacroOutput {
        MacroOutput(self.0.clone())
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

fn string_to_key_presses(s: &str) -> Vec<KeyPress> {
    s.chars()
        .map(|c| KeyPress::new(requires_shift(c), char_to_key(c)))
        .collect()
}
