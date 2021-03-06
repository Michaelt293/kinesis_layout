use std::fmt;

use either::*;

use crate::keys::*;

/// Models the output of a macro. A macro is represented as a vector of either a vector of
/// keypresses or a shortcut. This allows complex macros to be defined. For example, a
/// macro which outputs some text, followed by a keyboard shortcut, followed by some more text.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct MacroOutput(Vec<Either<Vec<KeyPress>, Shortcut>>);

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
                    let mut temp_string = String::new();
                    let keypad = shortcut.keypad;

                    temp_string.push_str(
                        format!(
                            "{{{}}}",
                            KeyLayer::new(keypad, Key::NonModifier(shortcut.non_modifier))
                        )
                        .as_str(),
                    );

                    for key in shortcut.modifiers.iter() {
                        temp_string.insert_str(0, format!("{{-{}}}", key).as_str());
                        temp_string.push_str(format!("{{+{}}}", key).as_str());
                    }

                    string.push_str(temp_string.as_str());
                }
            }
        }

        write!(f, "{}", string)
    }
}

/// An intermediate datatype used to and construct a `MacroOutputTemp` value using the
/// builder pattern.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug, Default)]
pub struct MacroBuilder(Vec<MacroComponent>);

impl MacroBuilder {
    /// Creates an empty `MacroBuilder`.
    pub fn new() -> MacroBuilder {
        Default::default()
    }

    /// Creates an `MacroBuilder` from a string literal.
    pub fn from_string(s: &str) -> MacroBuilder {
        MacroBuilder(vec![MacroComponent::KeyPresses(string_to_key_presses(s))])
    }

    /// Adds a string literal.
    pub fn with_string(&mut self, s: &str) -> &mut MacroBuilder {
        self.0
            .push(MacroComponent::KeyPresses(string_to_key_presses(s)));
        self
    }

    /// Adds a shortcut.
    pub fn with_shortcut(&mut self, shortcut: Shortcut) -> &mut MacroBuilder {
        self.0.push(MacroComponent::Shortcut(shortcut));
        self
    }

    /// Moves the cursor up.
    pub fn cursor_up(&mut self, up: usize) -> &mut MacroBuilder {
        self.0.push(MacroComponent::KeyPresses(vec![
            KeyPress::not_shifted(
                NonModifier::UpArrow
            );
            up
        ]));
        self
    }

    /// Moves the cursor down.
    pub fn cursor_down(&mut self, down: usize) -> &mut MacroBuilder {
        self.0.push(MacroComponent::KeyPresses(vec![
            KeyPress::not_shifted(
                NonModifier::DownArrow
            );
            down
        ]));
        self
    }

    /// Moves the cursor left.
    pub fn cursor_left(&mut self, left: usize) -> &mut MacroBuilder {
        self.0.push(MacroComponent::KeyPresses(vec![
            KeyPress::not_shifted(
                NonModifier::LeftArrow
            );
            left
        ]));
        self
    }

    /// Moves the cursor right.
    pub fn cursor_right(&mut self, right: usize) -> &mut MacroBuilder {
        self.0.push(MacroComponent::KeyPresses(vec![
            KeyPress::not_shifted(
                NonModifier::RightArrow
            );
            right
        ]));
        self
    }

    /// Adds a system-agnostic command.
    pub fn with_command(&mut self, command: Command) -> &mut MacroBuilder {
        self.0.push(MacroComponent::Command(command));
        self
    }

    /// Converts a `MacroBuilder` to a `MacroOutputTemp`
    pub fn make(&self) -> MacroOutputTemp {
        MacroOutputTemp(self.0.clone())
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
        '`' | '~' => Backtick,
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

/// Indicates whether the keyboard layout will be used with a `PC` or `Mac`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum System {
    PC,
    Windows,
    Mac,
}

impl System {
    /// Indicates whether or not `System` is a Mac.
    pub fn is_mac(self) -> bool {
        self == System::Mac
    }
}

impl Default for System {
    fn default() -> Self {
        System::PC
    }
}

/// Commands are system-agnostic actions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Command {
    Copy,
    Paste,
    Cut,
    Undo,
    JumpForward,
    JumpBack,
    LineEnd,
    LineStart,
}

impl Command {
    /// Outputs a system-specific `Shortcut` for a given `Command`.
    fn to_shortcut(self, system: System) -> Shortcut {
        use crate::keys::Modifier::*;
        use crate::keys::NonModifier::*;

        match self {
            Command::Copy if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, C)
            }

            Command::Copy => Shortcut::keypad_off(btreeset! {LeftControl}, NonModifier::C),

            Command::Paste if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, V)
            }

            Command::Paste => Shortcut::keypad_off(btreeset! {LeftControl}, V),

            Command::Cut if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, X)
            }

            Command::Cut => Shortcut::keypad_off(btreeset! {LeftControl}, NonModifier::X),

            Command::Undo if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, Z)
            }

            Command::Undo => Shortcut::keypad_off(btreeset! {LeftControl}, Z),

            Command::JumpForward if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {LeftAlt}, RightArrow)
            }

            Command::JumpForward => Shortcut::keypad_off(btreeset! {LeftControl}, RightArrow),

            Command::JumpBack if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {LeftAlt}, LeftArrow)
            }

            Command::JumpBack => Shortcut::keypad_off(btreeset! {LeftControl}, C),

            Command::LineEnd if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, LeftArrow)
            }

            Command::LineEnd => Shortcut::keypad_off(btreeset! {}, End),

            Command::LineStart if system.is_mac() => {
                Shortcut::keypad_off(btreeset! {RightWindowsCommand}, RightArrow)
            }

            Command::LineStart => Shortcut::keypad_off(btreeset! {}, Home),
        }
    }
}

/// Newtype wrapping a vector of `MacroOutputTemp`. This datatype represents system agnostic
/// macro components.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct MacroOutputTemp(Vec<MacroComponent>);

impl MacroOutputTemp {
    pub fn to_macro_output(&self, system: System) -> MacroOutput {
        MacroOutput(self.0.iter().map(|x| x.to_either(system)).collect())
    }
}

/// A `MacroComponent` is either a vector of keypresses, shortcut or a command. `MacroComponent`
/// is system agnostic.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum MacroComponent {
    KeyPresses(Vec<KeyPress>),
    Shortcut(Shortcut),
    Command(Command),
}

impl MacroComponent {
    /// Converts system agnostic commands to system-specific keyboard shortcuts.
    fn to_either(&self, system: System) -> Either<Vec<KeyPress>, Shortcut> {
        match self {
            MacroComponent::KeyPresses(presses) => Left(presses.clone()),
            MacroComponent::Shortcut(shortcut) => Right(shortcut.clone()),
            MacroComponent::Command(command) => Right(command.to_shortcut(system)),
        }
    }
}
