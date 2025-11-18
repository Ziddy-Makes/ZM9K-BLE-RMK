use defmt::{info, unwrap};
use rmk::controller::{Controller, PollingController};
use rmk::channel::CONTROLLER_CHANNEL;
use rmk::channel::ControllerSub;
use rmk::event::ControllerEvent;
use rmk::ble::BleState;
use embassy_nrf::gpio::Output;


pub struct BleConnectionLedController<'d> {
    pin: Output<'d>,
    sub: ControllerSub,
    should_blink: bool,
    low_active: bool,
}

impl<'d> BleConnectionLedController<'d> {
    pub fn new(pin: Output<'d>, low_active: bool) -> Self {
        Self {
            pin,
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            should_blink: false,
            low_active,
        }
    }

    fn activate(&mut self) {
        if self.low_active {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }
    }

    fn deactivate(&mut self) {
        if self.low_active {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
    }
}

impl<'d> Controller for BleConnectionLedController<'d> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        if let ControllerEvent::BleState(_profile, state) = event {
            match state {
                BleState::Advertising => {
                    // Start blinking when advertising
                    info!("Advertising - Custom Controller");
                    self.should_blink = true;
                }
                BleState::Connected => {
                    // Stop blinking, turn on solid when connected
                    self.should_blink = false;
                    info!("Connected - Custom Controller");
                    self.activate();
                }
                BleState::None => {
                    // Turn off when not in BLE mode
                    self.should_blink = false;
                    info!("None - Custom Controller");
                    self.deactivate();
                }
            }
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'d> PollingController for BleConnectionLedController<'d> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_millis(500);

    async fn update(&mut self) {
        if self.should_blink {
            self.pin.toggle();
        }
    }
}