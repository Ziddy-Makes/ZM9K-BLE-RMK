use defmt::{info, unwrap};
use rmk::controller::{Controller, PollingController};
use rmk::channel::CONTROLLER_CHANNEL;
use rmk::channel::ControllerSub;
use rmk::event::ControllerEvent;
use rmk::ble::BleState;
use embassy_nrf::gpio::Output;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;
use embassy_nrf::spim::Spim;


pub struct BleConnectionLedController<'d, const N: usize> {
    ws2812: Ws2812<Spim<'d>>,
    power_pin: Output<'d>,
    sub: ControllerSub,
    should_blink: bool,
    leds_on: bool,
}

impl<'d, const N: usize> BleConnectionLedController<'d, N> {
    pub fn new(ws2812: Ws2812<Spim<'d>>, power_pin: Output<'d>) -> Self {
        Self {
            ws2812,
            power_pin,
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            should_blink: false,
            leds_on: false,
        }
    }


    fn set_all_leds_blue(&mut self) {
        self.power_pin.set_high();
        let data = [RGB8 { r: 0, g: 0, b: 15 }; N];
        let _ = self.ws2812.write(data.iter().cloned());
        self.leds_on = true;
    }

    fn set_all_leds_green(&mut self) {
        self.power_pin.set_high();
        let data = [RGB8 { r: 0, g: 15, b: 0 }; N];
        let _ = self.ws2812.write(data.iter().cloned());
        self.leds_on = true;
    }

    fn clear_all_leds(&mut self) {
        let data = [RGB8::default(); N];
        let _ = self.ws2812.write(data.iter().cloned());
        self.power_pin.set_low();
        self.leds_on = false;
    }
}

impl<'d, const N: usize> Controller for BleConnectionLedController<'d, N> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        if let ControllerEvent::BleState(_profile, state) = event {
            match state {
                BleState::Advertising => {
                    // Start blinking blue when advertising
                    info!("Advertising - Custom Controller");
                    self.should_blink = true;
                }
                BleState::Connected => {
                    // Stop blinking and blink green 3 times
                    self.should_blink = false;
                    info!("Connected - Custom Controller");

                    // Blink green 3 times
                    for _ in 0..3 {
                        self.set_all_leds_green();
                        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
                        self.clear_all_leds();
                        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
                    }
                }
                BleState::None => {
                    // Turn off LEDs when not in BLE mode
                    self.should_blink = false;
                    info!("None - Custom Controller");
                    self.clear_all_leds();
                }
            }
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'d, const N: usize> PollingController for BleConnectionLedController<'d, N> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_millis(700);

    async fn update(&mut self) {
        if self.should_blink {
            if self.leds_on {
                self.clear_all_leds();
            } else {
                self.set_all_leds_blue();
            }
        }
    }
}