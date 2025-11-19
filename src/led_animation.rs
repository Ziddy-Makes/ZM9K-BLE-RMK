use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;
use embassy_nrf::spim::Spim;
use embassy_time::Timer;

pub struct LedAnimator<'d, const N: usize> {
    ws2812: Ws2812<Spim<'d>>,
}

impl<'d, const N: usize> LedAnimator<'d, N> {
    pub fn new(ws2812: Ws2812<Spim<'d>>) -> Self {
        Self { ws2812 }
    }

    /// Bootup animation: wave effect from start to end
    pub async fn bootup_animation(&mut self) {
        // Wave effect - light up each LED in sequence
        for i in 0..N {
            let mut data = [RGB8::default(); N];
            for j in 0..=i {
                data[j] = RGB8 { r: 15, g: 5, b: 0 }; // Maybe Orange color
            }
            let _ = self.ws2812.write(data.iter().cloned());
            Timer::after_millis(100).await;
        }

        // Flash all LEDs white
        let data = [RGB8 { r: 0, g: 0, b: 15 }; N];
        let _ = self.ws2812.write(data.iter().cloned());
        Timer::after_millis(300).await;

        // Turn off all LEDs
        let data = [RGB8::default(); N];
        let _ = self.ws2812.write(data.iter().cloned());
        Timer::after_millis(50).await;
    }

    /// Return the ws2812 controller for use elsewhere
    pub fn take(self) -> Ws2812<Spim<'d>> {
        self.ws2812
    }
}
