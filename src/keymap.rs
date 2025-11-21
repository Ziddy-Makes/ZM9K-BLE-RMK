use rmk::keyboard_macros::{MacroOperation, define_macro_sequences};
use rmk::morse::Morse;
use rmk::types::action::{Action, EncoderAction, KeyAction, MorseMode, MorseProfile};
use rmk::types::keycode::KeyCode;
use rmk::types::modifier::ModifierCombination;
use rmk::{a, encoder, k, layer, lt, td, wm};

// Modifier combination aliases
const LCTRL: ModifierCombination = ModifierCombination::LCTRL;
const CTRL_ALT: ModifierCombination = ModifierCombination::new()
    .with_left_ctrl(true)
    .with_left_alt(true);
const CTRL_SHIFT_GUI: ModifierCombination = ModifierCombination::new()
    .with_left_ctrl(true)
    .with_left_shift(true)
    .with_left_gui(true);

pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 4;
pub(crate) const SIZE: usize = 16; // Rows * Cols
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [td!(3),               k!(B),                       k!(MediaPlayPause),      a!(No)],
            [wm!(Enter, CTRL_ALT), wm!(Left, CTRL_ALT),         wm!(Right, CTRL_ALT),    a!(No)],
            [wm!(Up, LCTRL),       wm!(Left, LCTRL),            wm!(Right, LCTRL),       a!(No)],
            [lt!(1, KbMute),       a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [td!(0),               td!(1),                      td!(2),                  a!(No)],
            [a!(No),               a!(No),                      k!(User7),              a!(No)],
            [k!(Bootloader),       k!(User5),                   k!(User6),               a!(No)],
            [a!(Transparent),      a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [a!(No),               wm!(E, CTRL_ALT),            wm!(T, CTRL_ALT),        a!(No)],
            [a!(No),               wm!(U, CTRL_ALT),            wm!(I, CTRL_ALT),        a!(No)],
            [a!(No),               wm!(J, CTRL_ALT),            wm!(K, CTRL_ALT),        a!(No)],
            [a!(Transparent),      a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [a!(No),               wm!(Kc4, CTRL_SHIFT_GUI),    a!(No),                  a!(No)],
            [wm!(D, CTRL_ALT),     wm!(F, CTRL_ALT),            wm!(G, CTRL_ALT),        a!(No)],
            [a!(No),               a!(No),                      a!(No),                  a!(No)],
            [a!(Transparent),      a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [k!(Grave),            k!(F1),                      k!(F2),                  a!(No)],
            [a!(No),               a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [k!(Grave),            k!(F1),                      k!(F2),                  a!(No)],
            [a!(No),               a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [k!(Grave),            k!(F1),                      k!(F2),                  a!(No)],
            [a!(No),               a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)]
        ]),
        layer!([
            [k!(Grave),            k!(F1),                      k!(F2),                  a!(No)],
            [a!(No),               a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)],
            [k!(CapsLock),         a!(No),                      a!(No),                  a!(No)]
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

/// Configure keyboard macros
/// This function sets up macro sequences that can be triggered using Action::TriggerMacro(index)
pub fn configure_macros(behavior_config: &mut rmk::config::BehaviorConfig) {
    // Use in Keymap array
    // KeyAction::Single(Action::TriggerMacro(0))

    use rmk::heapless::Vec;

    // Macro 0: Hold Ctrl+Shift+Alt+Gui, tap 4, release all modifiers
    let mut macro0 = Vec::<MacroOperation, 2048>::new();
    let _ = macro0.push(MacroOperation::Press(KeyCode::LCtrl));
    let _ = macro0.push(MacroOperation::Press(KeyCode::LShift));
    let _ = macro0.push(MacroOperation::Press(KeyCode::LGui));
    let _ = macro0.push(MacroOperation::Tap(KeyCode::Kc4));
    let _ = macro0.push(MacroOperation::Release(KeyCode::LGui));
    let _ = macro0.push(MacroOperation::Release(KeyCode::LShift));
    let _ = macro0.push(MacroOperation::Release(KeyCode::LCtrl));

    // Create macro sequences array and define them
    let macro_sequences = [macro0];
    let binary_macros = define_macro_sequences(&macro_sequences);
    behavior_config.keyboard_macros.macro_sequences = binary_macros;
}
