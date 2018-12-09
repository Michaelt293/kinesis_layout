use std::collections::BTreeSet;
use std::collections::HashMap;

use keys::*;
use layout::Layout;
use macros::*;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Configure {
    system: System,
    remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    macros: HashMap<Shortcut, MacroOutputTemp>,
}

impl Configure {
    pub fn new() -> Configure {
        Default::default()
    }

    pub fn set_system(&mut self, system: System) -> &mut Configure {
        self.system = system;
        self
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

    pub fn with_macro(
        &mut self,
        shortcut: Shortcut,
        macro_output: MacroOutputTemp,
    ) -> &mut Configure {
        self.macros.insert(shortcut, macro_output);
        self
    }

    pub fn invert_key(&mut self, key: NonModifier) -> &mut Configure {
        self.macros.insert(
            Shortcut::keypad_off(BTreeSet::new(), key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_off(btreeset!{Modifier::RightShift}, key))
                .make(),
        );
        self.macros.insert(
            Shortcut::keypad_off(btreeset!{Modifier::RightShift}, key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_off(BTreeSet::new(), key))
                .make(),
        );
        self
    }

    pub fn invert_keypad_key(&mut self, key: NonModifier) -> &mut Configure {
        self.macros.insert(
            Shortcut::keypad_on(BTreeSet::new(), key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_on(btreeset!{Modifier::RightShift}, key))
                .make(),
        );
        self.macros.insert(
            Shortcut::keypad_on(btreeset!{Modifier::RightShift}, key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_on(BTreeSet::new(), key))
                .make(),
        );
        self
    }

    pub fn invert_numbers(&mut self) -> &mut Configure {
        use self::NonModifier::*;

        self.invert_key(One);
        self.invert_key(Two);
        self.invert_key(Three);
        self.invert_key(Four);
        self.invert_key(Five);
        self.invert_key(Six);
        self.invert_key(Seven);
        self.invert_key(Eight);
        self.invert_key(Nine);
        self
    }

    pub fn make(&self) -> Layout {
        Layout {
            remappings: self.remappings.clone(),
            macros: self
                .macros
                .iter()
                .map(|(k, v)| (k.clone(), v.to_macro_output(self.system)))
                .collect(),
        }
    }
}
