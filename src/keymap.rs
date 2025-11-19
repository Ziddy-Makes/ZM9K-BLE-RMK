use rmk::morse::Morse;
use rmk::types::action::{Action, EncoderAction, KeyAction, MorseMode, MorseProfile};
use rmk::types::modifier::ModifierCombination;
use rmk::{a, encoder, k, layer, lt, td, wm};
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 4;
pub(crate) const SIZE: usize = 16; // Rows * Cols
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A),                                                                           k!(B),                                                                            k!(MediaPlayPause),                                                                    a!(No)],
            [wm!(Enter, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)), wm!(Left, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),  wm!(Right, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),       a!(No)],
            [wm!(Up, ModifierCombination::LCTRL),                                             wm!(Left, ModifierCombination::LCTRL),                                            wm!(Right, ModifierCombination::LCTRL),                                                a!(No)],
            [lt!(1, KbMute),                                                                  a!(No),                                                                           a!(No),                                                                                a!(No)]
        ]),
        layer!([
            [td!(0),              td!(1),              td!(2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(Bootloader),      k!(User5),           k!(User6),           a!(No)],
            [a!(Transparent),     a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(G),               k!(G),               k!(G),               a!(No)],
            [k!(G),               a!(No),              a!(No),              a!(No)],
            [k!(G),               a!(No),              a!(No),              a!(No)],
            [a!(Transparent),     a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(Grave),           k!(F1),              k!(F2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(Grave),           k!(F1),              k!(F2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(Grave),           k!(F1),              k!(F2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(Grave),           k!(F1),              k!(F2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)]
        ]),
        layer!([
            [k!(Grave),           k!(F1),              k!(F2),              a!(No)],
            [a!(No),              a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)],
            [k!(CapsLock),        a!(No),              a!(No),              a!(No)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [encoder!(k!(KbVolumeUp), k!(KbVolumeDown))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
        [encoder!(k!(No), k!(No))],
    ]
}

/// Configure tapdance behaviors
/// This function sets up tapdance configurations that can be referenced in the keymap using td!(index)
pub fn configure_tapdance(behavior_config: &mut rmk::config::BehaviorConfig) {
    use rmk::morse::{HOLD, TAP};
    use rmk::types::keycode::KeyCode;

    // Example tapdance 0: Single tap = Space, Double tap = Enter, Hold = Shift
    let mut td0 = Morse::default();
    td0.profile = MorseProfile::new(
        None,                    // Use default unilateral_tap
        Some(MorseMode::Normal), // Normal mode
        Some(200),               // 200ms hold timeout
        Some(200),               // 200ms gap timeout
    );
    td0.put(TAP, Action::Key(KeyCode::User0));
    td0.put(HOLD, Action::Key(KeyCode::User5));

    //////////////////////////////////////////////////////////////////////////////

    // Example tapdance 1: Single tap = Escape, Double tap = Grave, Hold = Layer 1
    let mut td1 = Morse::default();
    td1.profile = MorseProfile::new(None, Some(MorseMode::Normal), Some(200), Some(200));
    td1.put(TAP, Action::Key(KeyCode::User1));
    td1.put(HOLD, Action::Key(KeyCode::User5));

    //////////////////////////////////////////////////////////////////////////////

    // Example tapdance 2: Single tap = Tab, Double tap = Backspace, Hold = Ctrl
    let mut td2 = Morse::default();
    td2.profile = MorseProfile::new(None, Some(MorseMode::Normal), Some(200), Some(200));
    td2.put(TAP, Action::Key(KeyCode::User2));
    td2.put(HOLD, Action::Key(KeyCode::User5));

    //////////////////////////////////////////////////////////////////////////////

    // Add tapdance configurations to behavior_config
    let _ = behavior_config.morse.morses.push(td0);
    let _ = behavior_config.morse.morses.push(td1);
    let _ = behavior_config.morse.morses.push(td2);
}
