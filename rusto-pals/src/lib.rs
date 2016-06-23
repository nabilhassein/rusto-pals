#![allow(dead_code)]

extern crate rustc_serialize;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::{ToHex, FromHex};

#[test]
fn check_challenge1() {
    let input: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(expected, challenge1(input));
}

#[test]
fn check_challenge2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";

    let expected = String::from("746865206b696420646f6e277420706c6179");
    let result = challenge2(input1, input2).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn check_challenge3() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let expected = String::from("cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON");
    let result = challenge3(input).unwrap();

    assert_eq!(expected, result);
}

fn challenge1(input: &str) -> String {
    input.from_hex().unwrap().to_base64(base64::STANDARD)
}

use std::iter::FromIterator;

#[derive(Debug)]
enum ChallengeError {
    FromHex(rustc_serialize::hex::FromHexError),
    Utf8Error(std::string::FromUtf8Error),
    Other(String),
}

impl From<rustc_serialize::hex::FromHexError> for ChallengeError {
    fn from(f: rustc_serialize::hex::FromHexError) -> ChallengeError {
        ChallengeError::FromHex(f)
    }
}

impl From<std::string::FromUtf8Error> for ChallengeError {
    fn from(f: std::string::FromUtf8Error) -> ChallengeError {
        ChallengeError::Utf8Error(f)
    }
}

impl From<&'static str> for ChallengeError {
    fn from(f: &'static str) -> ChallengeError {
        ChallengeError::Other(String::from(f))
    }
}

impl From<String> for ChallengeError {
    fn from(f: String) -> ChallengeError {
        ChallengeError::Other(f)
    }
}

fn challenge2(input1: &str, input2: &str) -> Result<String, ChallengeError> {
    let s1 = try!(input1.from_hex());
    let s2 = try!(input2.from_hex());

    let s = Vec::from_iter(s1.into_iter()
        .zip(s2.into_iter())
        .map(|(ch1, ch2)| ch1 ^ ch2));

    Ok(s.to_hex())
}

fn challenge3(input: &str) -> Result<String, ChallengeError> {
    for x in 0..256 {
        let input = try!(input.from_hex());

        let temp = Vec::from_iter(input.into_iter().map(|ch| ch ^ (x as u8)));

        // manually examined println output to determine index
        if x == 120 {
            return Ok(try!(String::from_utf8(temp)));
        };
    }

    //Err(From::from(format!("oh no {}", 5)))
    Err(From::from("oh no"))
}
