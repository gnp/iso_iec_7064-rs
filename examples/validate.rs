use iso_iec_7064::pure::PureSystem;
use iso_iec_7064::system::System;
use iso_iec_7064::Mod11_2;

fn main() -> () {
    let string = "07940";
    let result = Mod11_2::validate_string(string);
    if result {
        println!("{:?} IS valid according to {}", string, Mod11_2::name());
    } else {
        println!("{:?} is NOT valid according to {}", string, Mod11_2::name());
    }
}
