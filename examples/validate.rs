use iso_iec_7064::System;
use iso_iec_7064::MOD_11_2;

fn main() {
    let string = "07940";
    let result = MOD_11_2.validate_string(string);
    if result {
        println!("{:?} IS valid according to {}", string, MOD_11_2.name());
    } else {
        println!("{:?} is NOT valid according to {}", string, MOD_11_2.name());
    }
}
