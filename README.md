# kinesis_layout

This project allows keyboard layouts for the Kinesis Advantage 2 to be generated programmatically using Rust. Support for remapping keys and macros is provided by `kinesis_layout`. The aim is that by using a statically-typed programming language, only valid keyboard layouts should compile (note: run-time checks may need to be implemented to guard against edge cases).

As an example, the following layout in `kinesis_layout` -
 
```rust
     let layout: Layout = Configure::new()
         .with_remappings(colemak())
         .remap(Key::NonModifier(A), Key::NonModifier(LeftArrow))
         .remap_keypad(Key::NonModifier(Enter), Key::NonModifier(Space))
         .dead_key(Key::NonModifier(Backtick))
         .with_macro(
             Shortcut::keypad_on(BTreeSet::new(), C),
             MacroOutput::from_string("www.test.com, THANKS"),
         ).make();
```

produces the following keyboard layout -

```text
[J]>[N]
[K]>[E]
[`]>[null]
[F]>[T]
[;]>[O]
[S]>[R]
[D]>[S]
[I]>[U]
[kp-enter]>[kp0]
[P]>[;]
[O]>[Y]
[N]>[K]
[A]>[left]
[E]>[F]
[U]>[L]
[Y]>[J]
[L]>[I]
[R]>[P]
[T]>[G]
[G]>[D]
{kp-c}>{w}{w}{w}{.}{t}{e}{s}{t}{.}{c}{o}{m}{,}{space}{-lshift}{t}{h}{a}{n}{k}{s}{+lshift}
```

`kinesis_layout` is a work in progress but the ground work is now complete. Any suggestions or pull requests are welcome!
