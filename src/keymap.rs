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
            [td!(3),                                                                                 k!(B),                                                                                 k!(MediaPlayPause),                                                                                 a!(No)],
            [wm!(Enter, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),        wm!(Left, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                wm!(Right, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                a!(No)],
            [wm!(Up, ModifierCombination::LCTRL),                                                    wm!(Left, ModifierCombination::LCTRL),                                                         wm!(Right, ModifierCombination::LCTRL),                                                         a!(No)],
            [lt!(1, KbMute),                                                                         a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [td!(0),                                                                                 td!(1),                                                                                        td!(2),                                                                                         a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(Bootloader),                                                                         k!(User5),                                                                                     k!(User6),                                                                                      a!(No)],
            [a!(Transparent),                                                                        a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [a!(No),                                                                                 wm!(E, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                  wm!(T, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                    a!(No)],
            [a!(No),                                                                                 wm!(U, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                  wm!(I, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                    a!(No)],
            [a!(No),                                                                                 wm!(J, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                  wm!(K, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                    a!(No)],
            [a!(Transparent),                                                                        a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [a!(No),                                                                                 wm!(Kc4, ModifierCombination::new().with_left_ctrl(true).with_left_shift(true).with_left_alt(true)), a!(No),                                                                                   a!(No)],
            [wm!(D, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),            wm!(F, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                  wm!(G, ModifierCombination::new().with_left_ctrl(true).with_left_alt(true)),                    a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [a!(Transparent),                                                                        a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [k!(Grave),                                                                              k!(F1),                                                                                        k!(F2),                                                                                         a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [k!(Grave),                                                                              k!(F1),                                                                                        k!(F2),                                                                                         a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [k!(Grave),                                                                              k!(F1),                                                                                        k!(F2),                                                                                         a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)]
        ]),
        layer!([
            [k!(Grave),                                                                              k!(F1),                                                                                        k!(F2),                                                                                         a!(No)],
            [a!(No),                                                                                 a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)],
            [k!(CapsLock),                                                                           a!(No),                                                                                        a!(No),                                                                                         a!(No)]
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

    // Tapdance 3: Tap = One Shot Layer 3, Hold = Momentary Layer 2
    let mut td3 = Morse::default();
    td3.profile = MorseProfile::new(None, Some(MorseMode::Normal), Some(200), Some(200));
    td3.put(TAP, Action::OneShotLayer(3));
    td3.put(HOLD, Action::LayerOn(2));

    //////////////////////////////////////////////////////////////////////////////

    // Add tapdance configurations to behavior_config
    let _ = behavior_config.morse.morses.push(td0);
    let _ = behavior_config.morse.morses.push(td1);
    let _ = behavior_config.morse.morses.push(td2);
    let _ = behavior_config.morse.morses.push(td3);
}
