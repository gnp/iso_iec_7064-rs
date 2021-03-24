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
