#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

fn split_float(n: f32, power: u32) -> (u32, u32) {
    let m = (n * power as f32) as u32;
    (m / power, m % power)
}

macro_rules! writeln_float {
    ($s: expr, $n: expr, $p: expr) => {{
        let (b, c) = split_float($n, $p);
        ufmt::uwriteln!($s, "{}.{}", b, c).void_unwrap();
    }}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // let mut led = pins.d13.into_output();

    let r = [1_f32; 1];
    let mut f = [0_f32; 2];

    let digits = 4;
    let pow = 10u32.pow(digits);

    // In this loop, [k-a] will be represented by [a]
    loop {
        f[0] = 0.99 * f[1] + 0.00995 * r[0];

        // writeln_float!(&mut serial, f[0], pow);

        f.rotate_left(1);
        // f[1] = f[0];
        arduino_hal::delay_ms(100);
    }
}
