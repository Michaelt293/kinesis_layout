use std::collections::HashMap;

use keys::*;

pub fn colemak() -> HashMap<KeyLayer, Option<KeyLayer>> {
    use self::NonModifier::*;

    [
        (
            KeyLayer::off(Key::NonModifier(T)),
            Some(KeyLayer::off(Key::NonModifier(G))),
        ),
        (
            KeyLayer::off(Key::NonModifier(R)),
            Some(KeyLayer::off(Key::NonModifier(P))),
        ),
        (
            KeyLayer::off(Key::NonModifier(E)),
            Some(KeyLayer::off(Key::NonModifier(F))),
        ),
        (
            KeyLayer::off(Key::NonModifier(G)),
            Some(KeyLayer::off(Key::NonModifier(D))),
        ),
        (
            KeyLayer::off(Key::NonModifier(F)),
            Some(KeyLayer::off(Key::NonModifier(T))),
        ),
        (
            KeyLayer::off(Key::NonModifier(D)),
            Some(KeyLayer::off(Key::NonModifier(S))),
        ),
        (
            KeyLayer::off(Key::NonModifier(S)),
            Some(KeyLayer::off(Key::NonModifier(R))),
        ),
        (
            KeyLayer::off(Key::NonModifier(Y)),
            Some(KeyLayer::off(Key::NonModifier(J))),
        ),
        (
            KeyLayer::off(Key::NonModifier(U)),
            Some(KeyLayer::off(Key::NonModifier(L))),
        ),
        (
            KeyLayer::off(Key::NonModifier(I)),
            Some(KeyLayer::off(Key::NonModifier(U))),
        ),
        (
            KeyLayer::off(Key::NonModifier(O)),
            Some(KeyLayer::off(Key::NonModifier(Y))),
        ),
        (
            KeyLayer::off(Key::NonModifier(P)),
            Some(KeyLayer::off(Key::NonModifier(SemiColon))),
        ),
        (
            KeyLayer::off(Key::NonModifier(J)),
            Some(KeyLayer::off(Key::NonModifier(N))),
        ),
        (
            KeyLayer::off(Key::NonModifier(K)),
            Some(KeyLayer::off(Key::NonModifier(E))),
        ),
        (
            KeyLayer::off(Key::NonModifier(L)),
            Some(KeyLayer::off(Key::NonModifier(I))),
        ),
        (
            KeyLayer::off(Key::NonModifier(SemiColon)),
            Some(KeyLayer::off(Key::NonModifier(O))),
        ),
        (
            KeyLayer::off(Key::NonModifier(N)),
            Some(KeyLayer::off(Key::NonModifier(K))),
        ),
    ]
        .iter()
        .cloned()
        .collect()
}

//[T]>[G]
//[R]>[P]
//[E]>[F]
//[G]>[D]
//[F]>[T]
//[D]>[S]
//[S]>[R]
//[Y]>[J]
//[U]>[L]
//[I]>[U]
//[O]>[Y]
//[P]>[;]
//[J]>[N]
//[K]>[E]
//[L]>[I]
//[;]>[O]
//[N]>[K]
