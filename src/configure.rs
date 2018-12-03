use std::collections::HashMap;

use keys::{Key, KeyLayer, Shortcut};
use layout::Layout;
use macros::MacroOutput;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Configure {
    remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    macros: HashMap<Shortcut, MacroOutput>,
}

impl Configure {
    pub fn new() -> Configure {
        Default::default()
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
        self.remappings.insert(KeyLayer::off(key), None);
        self
    }

    pub fn keypad_dead_key(&mut self, key: Key) -> &mut Configure {
        self.remappings.insert(KeyLayer::on(key), None);
        self
    }

    pub fn remove_remap(&mut self, key: Key) -> &mut Configure {
        self.remappings.remove(&KeyLayer::off(key));
        self
    }

    pub fn remove_remap_keypad(&mut self, key: Key) -> &mut Configure {
        self.remappings.remove(&KeyLayer::on(key));
        self
    }

    pub fn with_macro(&mut self, shortcut: Shortcut, macro_output: MacroOutput) -> &mut Configure {
        self.macros.insert(shortcut, macro_output);
        self
    }

    pub fn make(&self) -> Layout {
        Layout {
            remappings: self.remappings.clone(),
            macros: self.macros.clone(),
        }
    }
}
