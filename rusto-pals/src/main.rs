extern crate rustc_serialize;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

fn main() {
    let input: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", challenge1(input));
}

fn challenge1(input: &str) -> String {
    input.from_hex().unwrap().to_base64(base64::STANDARD)
}
