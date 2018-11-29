use std::collections::HashMap;
use std::fmt;

use keys::*;
use macros::*;

pub struct Configure {
    remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    macros: HashMap<Shortcut, Vec<MacroOutput>>,
}

impl Configure {
    pub fn new() -> Configure {
        Configure {
            remappings: HashMap::new(),
            macros: HashMap::new(),
        }
    }

    pub fn remap(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remappings
            .insert(KeyLayer::off(old_key), Some(KeyLayer::off(new_key)));
        self
    }

    pub fn remap_keypad(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remappings
            .insert(KeyLayer::on(old_key), Some(KeyLayer::on(new_key)));
        self
    }

    pub fn remap_all(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remap(old_key.clone(), new_key.clone());
        self.remap_keypad(old_key, new_key);
        self
    }

    pub fn remap_permissive(&mut self, old_key: KeyLayer, new_key: KeyLayer) -> &mut Configure {
        self.remappings.insert(old_key, Some(new_key));
        self
    }

    pub fn with_remappings(
        &mut self,
        remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    ) -> &mut Configure {
        self.remappings.extend(remappings);
        self
    }

    pub fn dead_key(&mut self, key: Key) -> &mut Configure {
        self.remappings
            .insert(KeyLayer::off(key), None);
        self
    }

    pub fn keypad_dead_key(&mut self, key: Key) -> &mut Configure {
        self.remappings.insert(KeyLayer::on(key), None);
        self
    }

    pub fn remove_remap(&mut self, key: Key) -> &mut Configure{
        self.remappings.remove(&KeyLayer::off(key));
        self
    }

    pub fn remove_remap_keypad(&mut self, key: Key) -> &mut Configure{
        self.remappings.remove(&KeyLayer::on(key));
        self
    }

    pub fn with_macro(
        &mut self,
        shortcut: Shortcut,
        macro_output: MacroOutput,
    ) -> &mut Configure {
        self.macros.insert(shortcut, vec![macro_output]);
        self
    }

    pub fn make(&self) -> Layout {
        Layout {
            remappings: self.remappings.clone(),
            macros: self.macros.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Layout {
    remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    macros: HashMap<Shortcut, Vec<MacroOutput>>,
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            remappings: HashMap::new(),
            macros: HashMap::new(),
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut remappings: Vec<(&KeyLayer, &Option<KeyLayer>)> = self.remappings.iter().collect();
        remappings.sort_unstable();

        let mut mappings: Vec<String> = Vec::new();

        for (k, v) in self.remappings.iter() {
            let value = match v {
                None => format!("{}", "null"),
                Some(key) => format!("{}", key),
            };

            mappings.push(format!("[{}]>[{}]", k, value));
        }

        for (k, v) in self.macros.iter() {
            let mut value = String::new();
            for m in v {
                value.push_str(format!("{}", m).as_str())
            }

            mappings.push(format!("{}>{}", k, value).to_lowercase());
        }

        write!(f, "{}", mappings.join("\n"))
    }
}
