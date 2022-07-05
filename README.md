Rust Controller
==================
A project written in Rust for Arduino Uno with the purpose of being a feedback controller for any system. This project proposes a template implementation for discrete systems.

## Usage
If you don't have them already, install [`cargo-generate`] and [`ravedude`]:

```bash
cargo install cargo-generate
cargo install ravedude
```

You should be able to just

```bash
cargo run
```

and see a blinky flashed to your board!

[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/next/ravedude

## Example

As an example let's implement the system for a pendulum with a propeller attached to its end. The objective would be to control the pendulum's angle relative to the "down" direction, using the motor voltage as the control variable.

The system described above can be modelled the following way:

$$ A(x) = a_x \cdot x + b_x \cdot x^2 $$

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
