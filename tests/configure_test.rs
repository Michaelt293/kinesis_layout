extern crate indoc;
#[macro_use]
extern crate maplit;

extern crate kinesis_layout;

use indoc::indoc;

use kinesis_layout::configure::*;
use kinesis_layout::keys::*;
use kinesis_layout::layout::*;
use kinesis_layout::macros::*;

#[test]
fn layout_generation_test() {
    use Modifier::*;
    use NonModifier::*;

    let layout: Layout = Configure::new()
        .with_remappings(colemak())
        .remap(Key::NonModifier(A), Key::NonModifier(LeftArrow))
        .remap_keypad(Key::NonModifier(Enter), Key::NonModifier(Space))
        .dead_key(Key::NonModifier(Backtick))
        .invert_numbers()
        .with_macro(
            Shortcut::keypad_off(btreeset! {RightShift, LeftAlt}, T),
            MacroBuilder::from_string("www.test.com\nTHANKS")
                .cursor_left(6)
                .make(),
        )
        .with_macro(
            Shortcut::keypad_off(btreeset! {RightShift, LeftAlt}, I),
            MacroBuilder::new()
                .with_command(Command::LineEnd)
                .with_string("if  {\n")
                .cursor_down(1)
                .with_string(" else  {\n")
                .cursor_up(3)
                .with_shortcut(Shortcut::keypad_off(
                    btreeset! {LeftWindowsCommand},
                    RightArrow,
                ))
                .cursor_left(2)
                .make(),
        )
        .make();

    let output =
    indoc!("[`]>[null]
            [A]>[left]
            [D]>[S]
            [E]>[F]
            [F]>[T]
            [G]>[D]
            [I]>[U]
            [J]>[N]
            [K]>[E]
            [L]>[I]
            [N]>[K]
            [O]>[Y]
            [P]>[;]
            [R]>[P]
            [S]>[R]
            [T]>[G]
            [U]>[L]
            [Y]>[J]
            [;]>[O]
            [kp-enter]>[kp0]
            {1}>{-rshift}{1}{+rshift}
            {2}>{-rshift}{2}{+rshift}
            {3}>{-rshift}{3}{+rshift}
            {4}>{-rshift}{4}{+rshift}
            {5}>{-rshift}{5}{+rshift}
            {6}>{-rshift}{6}{+rshift}
            {7}>{-rshift}{7}{+rshift}
            {8}>{-rshift}{8}{+rshift}
            {9}>{-rshift}{9}{+rshift}
            {rshift}{1}>{1}
            {rshift}{2}>{2}
            {rshift}{3}>{3}
            {rshift}{4}>{4}
            {rshift}{5}>{5}
            {rshift}{6}>{6}
            {rshift}{7}>{7}
            {rshift}{8}>{8}
            {rshift}{9}>{9}
            {rshift}{lalt}{i}>{end}{i}{f}{space}{space}{-lshift}{obrack}{+lshift}{enter}{down}{space}{e}{l}{s}{e}{space}{space}{-lshift}{obrack}{+lshift}{enter}{up}{up}{up}{-lwin}{right}{+lwin}{left}{left}
            {rshift}{lalt}{t}>{w}{w}{w}{.}{t}{e}{s}{t}{.}{c}{o}{m}{enter}{-lshift}{t}{h}{a}{n}{k}{s}{+lshift}{left}{left}{left}{left}{left}{left}");

    assert_eq!(format!("{}", layout), output);
}
