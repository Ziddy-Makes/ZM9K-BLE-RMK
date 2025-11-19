use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, lt, th};
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 4;
pub(crate) const SIZE: usize = 16; // Rows * Cols
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A),              k!(B),              k!(C),              a!(No)],
            [k!(D),              k!(E),              k!(F),              a!(No)],
            [k!(G),              k!(H),              k!(I),              a!(No)],
            [lt!(1, KbMute),     a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(User0),          k!(User1),          k!(User2),          k!(User3)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(Bootloader),     k!(User5),          k!(User6),          a!(No)],
            [a!(Transparent),    a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
        layer!([
            [k!(Grave),          k!(F1),             k!(F2),             a!(No)],
            [a!(No),             a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)],
            [k!(CapsLock),       a!(No),             a!(No),             a!(No)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(No), k!(No))
        ],
        [
            encoder!(k!(No), k!(No))

        ],
        [
            encoder!(k!(No), k!(No))

        ],
        [
            encoder!(k!(No), k!(No))

        ],
        [
            encoder!(k!(No), k!(No))

        ],
        [
            encoder!(k!(No), k!(No))

        ],
        [
            encoder!(k!(No), k!(No))

        ],
    ]
}
