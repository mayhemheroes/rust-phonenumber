#![no_main]
use phonenumber::parse;
use phonenumber::Mode;

use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Arbitrary)]
struct FuzzInput {
    number: String,
    country_code: String,
    format_mode: u8,
}

fuzz_target!(|data: &[u8]| {
    let mut data = data.clone();

    let input = match FuzzInput::arbitrary(&mut Unstructured::new(&mut data)) {
        Ok(input) => input,
        Err(_) => return,
    };

    let country_code = match input.country_code.parse::<phonenumber::country::Id>() {
        Ok(country_code) => country_code,
        Err(_) => return,
    };

    let number = match parse(Some(country_code), &input.number) {
        Ok(number) => number,
        Err(_) => return,
    };

    let mode = match input.format_mode % 4 {
        0 => Mode::International,
        1 => Mode::National,
        2 => Mode::Rfc3966,
        _ => Mode::E164,
    };

    let _ = number.format().mode(mode);
});