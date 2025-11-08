use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer};
pub(crate) const COL: usize = 3;
pub(crate) const ROW: usize = 3;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Grave), k!(Kc1), k!(Kc2)],
            [k!(Tab), k!(Q), k!(W)],
            [k!(Escape), k!(A), k!(S)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2)],
            [a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
    ]
}
