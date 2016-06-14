extern crate rustc_serialize;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::{ToHex, FromHex};

fn main() {
    {
        //let input: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        //println!("{}", challenge1(input));
    }
    
    {
        // let input1 = "1c0111001f010100061a024b53535009181c";
        // let input2 = "686974207468652062756c6c277320657965";
        // println!("{}", challenge2(input1, input2));
    }

    {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        println!("{}", challenge3(input));
    }
}

fn challenge1(input: &str) -> String {
    input.from_hex().unwrap().to_base64(base64::STANDARD)
}

fn challenge2(input1: &str, input2: &str) -> String {
    String::from_utf8(
        input1.from_hex().unwrap().iter()
        .zip(input2.from_hex().unwrap().iter())
        .map(|(&ch1, &ch2)| {
            ch1 ^ ch2
        })
        .collect()).unwrap().as_bytes().to_hex()
}

fn challenge3(input: &str) -> String {
    for x in 0..256 {
        let temp = String::from_utf8(
            input.from_hex().unwrap().iter().map(
                |&ch| ch ^ (x as u8)
            ).collect());

        let ans = match temp {
            Ok(result) => result, // println!("{}: {}", x, result),
            Err(error) => "".to_string() // println!("{}: {}", x, error)
        };

        // manually examined println output to determine index
        if x == 120 { return ans };
    }

    unreachable!()
}
