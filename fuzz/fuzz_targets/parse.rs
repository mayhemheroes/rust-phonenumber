#![no_main]
use phonenumber::parse;

use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Arbitrary)]
struct FuzzInput {
    number: String,
    country_code: String,
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

    let _ = parse(Some(country_code), &input.number);
});