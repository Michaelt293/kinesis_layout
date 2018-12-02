#[macro_use]
extern crate maplit;

extern crate kinesis_layout;

use std::fs::File;
use std::io::Write;

use kinesis_layout::configure::*;
use kinesis_layout::keys::*;
use kinesis_layout::layout::*;
use kinesis_layout::macros::*;

fn main() {
    use Modifier::*;
    use NonModifier::*;

    let layout: Layout = Configure::new()
        .with_remappings(colemak())
        .remap(Key::NonModifier(A), Key::NonModifier(LeftArrow))
        .remap_keypad(Key::NonModifier(Enter), Key::NonModifier(Space))
        .dead_key(Key::NonModifier(Backtick))
        .with_macro(
            Shortcut::keypad_off(btreeset!{RightShift, LeftAlt}, T),
            MacroOutput::from_string_move_cursor("www.test.com\nTHANKS", 6),
        ).with_macros(
            Shortcut::keypad_on(btreeset!{LeftAlt}, V),
            vec![
                MacroOutput::shortcut_keypad_on(btreeset!{RightShift, LeftAlt}, T),
                MacroOutput::from_string("Hi"),
            ],
        ).make();

    let file_name: &str = "layout1";

    let mut f: File = File::create("layouts/".to_owned() + file_name)
        .expect(&("Unable to create file: ".to_owned() + file_name));

    f.write_all(format!("{}", layout).as_bytes())
        .expect("Unable to write data");
}
