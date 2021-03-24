iso_iec_7064
====

Conforming implementation of [ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
"Information technology &mdash; Security techniques &mdash; Check character systems" (The
Standard).

This crate implements all the _Check Character Systems_ specified in The Standard.

Note that the `mod_97_10` system is used by [ISO 17442-1:2020](https://www.iso.org/standard/78829.html)
"Financial services — Legal entity identifier (LEI) — Part 1: Assignment", which is implemented
by [the `lei` crate](https://crates.io/crates/lei).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iso_iec_7064 = "0.1"
```


## Example

```rust
use iso_iec_7064::{Mod11_2, PureCheckCharacterSystem};

fn main() -> () {
    let string = "07940";
    let result = Mod11_2::validate(string);
    if result {
        println!("{:?} IS valid according to {}", string, Mod11_2::NAME);
    } else {
        println!("{:?} is NOT valid according to {}", string, Mod11_2::NAME);
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
