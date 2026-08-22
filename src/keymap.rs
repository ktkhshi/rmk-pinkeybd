use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, lt, mo, user};

pub const ROW: usize = 11;
pub const COL: usize = 6;
pub const LAYERS: usize = 6;
pub const ENCODERS: usize = 1;

// Logical positions match the original ZMK matrix transform exactly. Row 5 is
// intentionally unused: the right half begins at row 6.
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; LAYERS] {
    [
        [
            [a!(No), k!(W), k!(E), k!(R), k!(T), k!(Escape)],
            [k!(Q), a!(No), k!(D), k!(F), k!(G), k!(RGui)],
            [k!(A), k!(S), a!(No), k!(V), k!(B), k!(Tab)],
            [k!(Z), k!(X), k!(C), a!(No), a!(No), a!(No)],
            [k!(LShift), k!(LCtrl), lt!(1, Language1), k!(Space), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(U), k!(I), k!(O), k!(P), a!(No)],
            [k!(Y), a!(No), k!(K), k!(L), k!(Semicolon), a!(No)],
            [k!(H), k!(J), a!(No), k!(Dot), k!(Slash), a!(No)],
            [k!(N), k!(M), k!(Comma), a!(No), mo!(3), a!(No)],
            [k!(Minus), k!(Enter), lt!(2, Language2), k!(RShift), a!(No), a!(No)],
        ],
        // Lower
        [
            [a!(No), k!(Comma), k!(End), k!(Minus), k!(Semicolon), k!(Escape)],
            [k!(Slash), a!(No), k!(D), k!(F), k!(G), k!(LCtrl)],
            [k!(Home), k!(S), a!(No), k!(Paste), k!(B), k!(Tab)],
            [k!(Z), k!(X), k!(C), a!(No), a!(No), a!(No)],
            [k!(LShift), k!(LAlt), a!(No), k!(Space), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Z), k!(Up), k!(LeftBracket), k!(RightBracket), a!(No)],
            [k!(Y), a!(No), k!(Down), k!(Right), k!(Delete), a!(No)],
            [k!(Backspace), k!(Left), a!(No), k!(Dot), k!(Z), a!(No)],
            [k!(N), k!(M), k!(Comma), a!(No), mo!(3), a!(No)],
            [k!(PrintScreen), k!(Enter), mo!(3), k!(RShift), a!(No), a!(No)],
        ],
        // Lower2
        [
            [a!(No), k!(Kc1), k!(Kc2), k!(Minus), k!(Semicolon), k!(Escape)],
            [a!(No), a!(No), k!(D), k!(F), k!(G), k!(LCtrl)],
            [k!(Home), k!(S), a!(No), k!(Paste), k!(B), k!(Tab)],
            [k!(Z), k!(X), k!(C), a!(No), a!(No), a!(No)],
            [k!(LAlt), k!(LShift), mo!(4), k!(Space), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(R), k!(PageUp), k!(LeftBracket), k!(RightBracket), a!(No)],
            [k!(L), a!(No), k!(PageDown), k!(End), k!(Delete), a!(No)],
            [k!(Backspace), k!(Home), a!(No), k!(Grave), k!(Insert), a!(No)],
            [k!(Quote), k!(Equal), k!(Backslash), a!(No), a!(No), a!(No)],
            [k!(F), k!(Enter), a!(No), k!(RShift), a!(No), a!(No)],
        ],
        // Number / Bluetooth-layer access
        [
            [a!(No), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), a!(No)],
            [k!(Kc1), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(LShift), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [mo!(5), k!(LShift), a!(No), k!(Space), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), a!(No)],
            [k!(Kc6), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(Grave), k!(Insert), a!(No)],
            [k!(Quote), k!(Equal), k!(Backslash), a!(No), a!(No), a!(No)],
            [a!(No), k!(Enter), a!(No), k!(RShift), a!(No), a!(No)],
        ],
        // Function
        [
            [a!(No), k!(F2), k!(F3), k!(F4), k!(F5), a!(No)],
            [k!(F1), a!(No), k!(F12), k!(F13), k!(F14), a!(No)],
            [k!(F6), k!(F7), a!(No), a!(No), a!(No), a!(No)],
            [k!(F11), k!(F12), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(F17), k!(F18), k!(F19), k!(F20), a!(No)],
            [k!(F16), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
        // BLE profiles: User0..5 select profiles, then next/previous/clear/output.
        [
            [a!(No), user!(1), user!(2), user!(3), user!(4), user!(5)],
            [user!(0), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), user!(8), a!(No), a!(No)],
            [a!(No), a!(No), user!(7), a!(No), user!(6), a!(No)],
            [a!(No), a!(No), a!(No), user!(9), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; ENCODERS]; LAYERS] {
    [[encoder!(k!(PageDown), k!(PageUp))]; LAYERS]
}
