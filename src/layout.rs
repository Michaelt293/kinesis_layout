use std::collections::HashMap;

use std::fmt;

use crate::keys::*;
use crate::macros::*;

/// `Layout` represents a keyboard layout including key remappings and macros. A `Layout`
///  should be constructed from a `Configuration` using the builder pattern.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Layout {
    pub remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    pub macros: HashMap<Shortcut, MacroOutput>,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut remappings: Vec<_> = self.remappings.iter().collect();
        remappings.sort_unstable();

        let mut mappings: Vec<String> = Vec::new();

        for (k, v) in remappings.iter() {
            let value = match v {
                None => "null".to_string(),
                Some(key) => format!("{}", key),
            };

            mappings.push(format!("[{}]>[{}]", k, value));
        }

        let mut macros: Vec<_> = self.macros.iter().collect();
        macros.sort_unstable();

        for (k, v) in macros.iter() {
            mappings.push(format!("{}>{}", k, v).to_lowercase());
        }

        write!(f, "{}", mappings.join("\n"))
    }
}

/// Key remappings for the `colemak` keyboard layout.
pub fn colemak() -> HashMap<KeyLayer, Option<KeyLayer>> {
    use self::NonModifier::*;

    hashmap! {
        KeyLayer::off(Key::NonModifier(T)) => Some(KeyLayer::off(Key::NonModifier(G))),
        KeyLayer::off(Key::NonModifier(R)) => Some(KeyLayer::off(Key::NonModifier(P))),
        KeyLayer::off(Key::NonModifier(E)) => Some(KeyLayer::off(Key::NonModifier(F))),
        KeyLayer::off(Key::NonModifier(G)) => Some(KeyLayer::off(Key::NonModifier(D))),
        KeyLayer::off(Key::NonModifier(F)) => Some(KeyLayer::off(Key::NonModifier(T))),
        KeyLayer::off(Key::NonModifier(D)) => Some(KeyLayer::off(Key::NonModifier(S))),
        KeyLayer::off(Key::NonModifier(S)) => Some(KeyLayer::off(Key::NonModifier(R))),
        KeyLayer::off(Key::NonModifier(Y)) => Some(KeyLayer::off(Key::NonModifier(J))),
        KeyLayer::off(Key::NonModifier(U)) => Some(KeyLayer::off(Key::NonModifier(L))),
        KeyLayer::off(Key::NonModifier(I)) => Some(KeyLayer::off(Key::NonModifier(U))),
        KeyLayer::off(Key::NonModifier(O)) => Some(KeyLayer::off(Key::NonModifier(Y))),
        KeyLayer::off(Key::NonModifier(P)) => Some(KeyLayer::off(Key::NonModifier(SemiColon))),
        KeyLayer::off(Key::NonModifier(J)) => Some(KeyLayer::off(Key::NonModifier(N))),
        KeyLayer::off(Key::NonModifier(K)) => Some(KeyLayer::off(Key::NonModifier(E))),
        KeyLayer::off(Key::NonModifier(L)) => Some(KeyLayer::off(Key::NonModifier(I))),
        KeyLayer::off(Key::NonModifier(SemiColon)) => Some(KeyLayer::off(Key::NonModifier(O))),
        KeyLayer::off(Key::NonModifier(N)) => Some(KeyLayer::off(Key::NonModifier(K)))
    }
}
