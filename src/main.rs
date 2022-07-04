#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    let r = [1.0; 1];
    let mut f = [0.0; 2];

    // In this loop, [k-a] will be represented by [a]
    loop {
        f[0] = 0.99 * f[1] + 0.00995 * r[0];

        // let res = buffer.format_finite(f[0]);


        let res = (f[0] * 1e5) as u64;
        let res1 = res / 100000;
        let res2 = res % 100000;
        ufmt::uwriteln!(&mut serial, "{}.{}", res1, res2)
            .void_unwrap();

        // f.rotate_left(1);  <-- This does the same as the line below (when `f.len() == 2`), but for some reason causes error in `avr-gcc` too
        f[1] = f[0];

        led.toggle();
        arduino_hal::delay_ms(10);
    }
}
