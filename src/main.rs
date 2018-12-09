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
        .invert_numbers()
        .with_macro(
            Shortcut::keypad_off(btreeset!{RightShift, LeftAlt}, T),
            MacroBuilder::new()
                .with_string("www.test.com\nTHANKS")
                .cursor_left(6)
                .make(),
        ).with_macro(
            Shortcut::keypad_off(btreeset!{RightShift, LeftAlt}, I),
            MacroBuilder::new()
                .with_string("if  {\n")
                .cursor_down(1)
                .with_string(" else  {\n")
                .cursor_up(3)
                .with_shortcut(Shortcut::keypad_off(
                    btreeset!{LeftWindowsCommand},
                    RightArrow,
                )).cursor_left(2)
                .make(),
        ).make();

    let file_name: &str = "layout1";

    let mut f: File = File::create("layouts/".to_owned() + file_name)
        .expect(&("Unable to create file: ".to_owned() + file_name));

    f.write_all(format!("{}", layout).as_bytes())
        .expect("Unable to write data");
}
