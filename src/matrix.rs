//! Pinkeybd's 6-line Charlieplex matrix with a logical-row offset.
//!
//! RMK's stock `BidirectionalMatrix` does not have row offsets. Split
//! peripherals need one, so this small adaptation preserves the original ZMK
//! transform: left is rows 0..4 and right is rows 6..10.
use embassy_nrf::gpio::{Flex, OutputDrive, Pull};
use embassy_time::Timer;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::debounce::{DebounceState, DebouncerTrait};
use rmk::event::KeyboardEvent;
use rmk::macros::input_device;
use rmk::matrix::KeyState;

pub const LOCAL_ROWS: usize = 5;
pub const COLS: usize = 6;
pub const PINS: usize = 6;

#[derive(Clone, Copy)]
enum ScanLocation {
    Pins(usize, usize), // (sampled pin, driven pin)
    Ignore,
}

#[input_device(publish = KeyboardEvent)]
pub struct PinkeybdMatrix<'d, const ROW_OFFSET: usize> {
    pins: [Flex<'d>; PINS],
    debouncer: DefaultDebouncer<LOCAL_ROWS, COLS>,
    key_state: [[KeyState; COLS]; LOCAL_ROWS],
    scan_pos: (usize, usize),
    scan_map: [[ScanLocation; COLS]; LOCAL_ROWS],
}

pub fn new<'d, const ROW_OFFSET: usize>(pins: [Flex<'d>; PINS]) -> PinkeybdMatrix<'d, ROW_OFFSET> {
    let scan_map = [
        [
            ScanLocation::Ignore,
            ScanLocation::Pins(1, 0),
            ScanLocation::Pins(2, 0),
            ScanLocation::Pins(3, 0),
            ScanLocation::Pins(4, 0),
            ScanLocation::Pins(5, 0),
        ],
        [
            ScanLocation::Pins(0, 1),
            ScanLocation::Ignore,
            ScanLocation::Pins(2, 1),
            ScanLocation::Pins(3, 1),
            ScanLocation::Pins(4, 1),
            ScanLocation::Pins(5, 1),
        ],
        [
            ScanLocation::Pins(0, 2),
            ScanLocation::Pins(1, 2),
            ScanLocation::Ignore,
            ScanLocation::Pins(3, 2),
            ScanLocation::Pins(4, 2),
            ScanLocation::Pins(5, 2),
        ],
        [
            ScanLocation::Pins(0, 3),
            ScanLocation::Pins(1, 3),
            ScanLocation::Pins(2, 3),
            ScanLocation::Ignore,
            ScanLocation::Pins(4, 3),
            ScanLocation::Pins(5, 3),
        ],
        [
            ScanLocation::Pins(0, 4),
            ScanLocation::Pins(1, 4),
            ScanLocation::Pins(2, 4),
            ScanLocation::Pins(3, 4),
            ScanLocation::Ignore,
            ScanLocation::Pins(5, 4),
        ],
    ];
    PinkeybdMatrix {
        pins,
        debouncer: DefaultDebouncer::new(),
        key_state: [[KeyState::new(); COLS]; LOCAL_ROWS],
        scan_pos: (0, 0),
        scan_map,
    }
}

impl<const ROW_OFFSET: usize> PinkeybdMatrix<'_, ROW_OFFSET> {
    async fn read_keyboard_event(&mut self) -> KeyboardEvent {
        loop {
            let (start_row, start_col) = self.scan_pos;
            for row in start_row..LOCAL_ROWS {
                let col_start = if row == start_row { start_col } else { 0 };
                for col in col_start..COLS {
                    let ScanLocation::Pins(input, output) = self.scan_map[row][col] else {
                        continue;
                    };
                    let [input_pin, output_pin] =
                        self.pins.get_disjoint_mut([input, output]).unwrap();
                    output_pin.set_as_output(OutputDrive::Standard);
                    output_pin.set_high();
                    Timer::after_micros(1).await;

                    let state = input_pin.is_high();
                    if let DebounceState::Debounced = self.debouncer.detect_change_with_debounce(
                        row,
                        col,
                        state,
                        &self.key_state[row][col],
                    ) {
                        self.key_state[row][col].toggle_pressed();
                        self.scan_pos = (row, col);
                        output_pin.set_low();
                        output_pin.set_as_input(Pull::Down);
                        return KeyboardEvent::key(
                            (row + ROW_OFFSET) as u8,
                            col as u8,
                            self.key_state[row][col].pressed,
                        );
                    }

                    output_pin.set_low();
                    output_pin.set_as_input(Pull::Down);
                    Timer::after_micros(1).await;
                }
            }
            self.scan_pos = (0, 0);
        }
    }
}
