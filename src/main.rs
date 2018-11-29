#[macro_use]
extern crate kinesis_layout;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

use kinesis_layout::configure::*;
use kinesis_layout::keys::*;
use kinesis_layout::layouts::*;
use kinesis_layout::macros::*;


fn main() {
    use NonModifier::*;

    let layout: Layout = Configure::new()
        .with_remappings(colemak())
        .remap(Key::NonModifier(A), Key::NonModifier(LeftArrow))
        .remap_keypad(Key::NonModifier(Enter), Key::NonModifier(Space))
        .dead_key(Key::NonModifier(Backtick))
        .with_macro(
            Shortcut::keypad_on(BTreeSet::new(), C),
            MacroOutput::from_string("www.test.com, THANKS"),
        ).make();

    let file_name: &str = "layout1";

    let mut f: File = File::create("layouts/".to_owned() + file_name)
        .expect(&("Unable to create file: ".to_owned() + file_name));

    f.write_all(format!("{}", layout).as_bytes())
        .expect("Unable to write data");
}
