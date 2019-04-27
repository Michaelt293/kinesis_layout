use std::collections::BTreeSet;
use std::collections::HashMap;

use crate::keys::*;
use crate::layout::Layout;
use crate::macros::*;

/// `Configure` is used with the builder pattern to configure a keyboard `Layout`.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Configure {
    system: System,
    remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    macros: HashMap<Shortcut, MacroOutputTemp>,
}

impl Configure {
    /// Creates a new `Configure` value  with `system` set to `PC` and
    /// no key remappings or macros.
    pub fn new() -> Configure {
        Default::default()
    }

    /// Used to set the system to either `PC` or `Mac`.
    pub fn set_system(&mut self, system: System) -> &mut Configure {
        self.system = system;
        self
    }

    /// Used to remap a single key.
    pub fn remap(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remappings
            .insert(KeyLayer::off(old_key), Some(KeyLayer::off(new_key)));
        self
    }

    /// Used to remap a single key in the keypad layer.
    pub fn remap_keypad(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remappings
            .insert(KeyLayer::on(old_key), Some(KeyLayer::on(new_key)));
        self
    }

    /// Used to remap a single key in both the top and keypad layers.
    pub fn remap_all(&mut self, old_key: Key, new_key: Key) -> &mut Configure {
        self.remap(old_key.clone(), new_key.clone());
        self.remap_keypad(old_key, new_key);
        self
    }

    /// Used to remap a single key. This method allows a key to be mapped between the top
    /// and keypad layers. If this functionality is not required, preferred the use of
    /// `remap`, `remap_keypad` and `remap_all` methods.
    pub fn remap_permissive(&mut self, old_key: KeyLayer, new_key: KeyLayer) -> &mut Configure {
        self.remappings.insert(old_key, Some(new_key));
        self
    }

    /// Pass in a `HashMap` of key remappings. This method is very useful when building a
    /// `Layout` using an alternative keyboard layout such as Dvorak or Colemak.
    pub fn with_remappings(
        &mut self,
        remappings: HashMap<KeyLayer, Option<KeyLayer>>,
    ) -> &mut Configure {
        self.remappings.extend(remappings);
        self
    }

    /// Creates a dead key.
    pub fn dead_key(&mut self, key: Key) -> &mut Configure {
        self.remappings.insert(KeyLayer::off(key), None);
        self
    }

    /// Creates a dead key in the keypad layer.
    pub fn keypad_dead_key(&mut self, key: Key) -> &mut Configure {
        self.remappings.insert(KeyLayer::on(key), None);
        self
    }

    /// Removes a remapping from the configuration. This method may be useful if, for example,
    /// you want to use an alternative keyboard layout such as Dvorak using the `with_remappings`
    /// method but want to remove a remapping.
    pub fn remove_remap(&mut self, key: Key) -> &mut Configure {
        self.remappings.remove(&KeyLayer::off(key));
        self
    }

    /// Removes a remapping from the keypad layer within the configuration. This method may be
    /// useful if, for example, you want to use an alternative keyboard layout such as Dvorak
    /// using the `with_remappings` method but want to remove a remapping.
    pub fn remove_remap_keypad(&mut self, key: Key) -> &mut Configure {
        self.remappings.remove(&KeyLayer::on(key));
        self
    }

    /// Used to add a macro to the keyboard layout. A macro requires a keypad shortcut and output
    /// upon triggering the keypad shortcut.
    pub fn with_macro(
        &mut self,
        shortcut: Shortcut,
        macro_output: MacroOutputTemp,
    ) -> &mut Configure {
        self.macros.insert(shortcut, macro_output);
        self
    }

    /// Inverts a key, i.e., inverting the key `5` means that the `%` symbol can be accessed
    /// without holding shift (`5` will require holding shift).
    pub fn invert_key(&mut self, key: NonModifier) -> &mut Configure {
        self.macros.insert(
            Shortcut::keypad_off(BTreeSet::new(), key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_off(btreeset! {Modifier::RightShift}, key))
                .make(),
        );
        self.macros.insert(
            Shortcut::keypad_off(btreeset! {Modifier::RightShift}, key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_off(BTreeSet::new(), key))
                .make(),
        );
        self
    }

    /// Inverts a key in the keypad layer, i.e., inverting the key `5` means that the `%` symbol
    /// can be accessed without holding shift (`5` will require holding shift).
    pub fn invert_keypad_key(&mut self, key: NonModifier) -> &mut Configure {
        self.macros.insert(
            Shortcut::keypad_on(BTreeSet::new(), key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_on(btreeset! {Modifier::RightShift}, key))
                .make(),
        );
        self.macros.insert(
            Shortcut::keypad_on(btreeset! {Modifier::RightShift}, key),
            MacroBuilder::new()
                .with_shortcut(Shortcut::keypad_on(BTreeSet::new(), key))
                .make(),
        );
        self
    }

    /// Inverts the number keys (0-9) so that accessing symbols does not require holding shift.
    /// May be useful for programmers who make heavy use of symbols.
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

    /// Converts a `Configuration` to a `Layout`. The `system` field of `Configuration` is used to
    /// create macros with the correct keyboard shortcuts.
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
