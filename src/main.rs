use serde_json::{self};
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    decode(encoded_value, 0).unwrap().0
}

fn decode(encoded_value: &str, index: usize) -> Option<(serde_json::Value, usize)> {
    let first_char = encoded_value.chars().nth(index).unwrap();
    if first_char.is_ascii_digit() {
        decode_string(encoded_value, index)
    } else if first_char == 'i' {
        decode_number(encoded_value, index)
    } else if first_char == 'l' {
        decode_list(encoded_value, index)
    } else if first_char == 'd' {
        decode_dict(encoded_value, index)
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_string(encoded_value: &str, index: usize) -> Option<(serde_json::Value, usize)> {
    let colon_index = encoded_value.get(index..)?.find(':')? + index;
    let number_string = &encoded_value[index..colon_index];
    let number = number_string.parse::<i64>().ok()? as usize;
    Some((
        serde_json::Value::String(
            encoded_value[colon_index + 1..colon_index + 1 + number].to_string(),
        ),
        colon_index + number + 1,
    ))
}

fn decode_number(encoded_value: &str, index: usize) -> Option<(serde_json::Value, usize)> {
    let e_index = encoded_value.get(index..)?.find('e')? + index;
    let number_string = &encoded_value[index + 1..e_index];
    let number = number_string.parse::<i64>().ok()?;
    Some((
        serde_json::Value::Number(serde_json::Number::from(number)),
        e_index + 1,
    ))
}

fn decode_list(encoded_value: &str, index: usize) -> Option<(serde_json::Value, usize)> {
    let mut res = Vec::new();
    let mut i = index + 1;
    while encoded_value.chars().nth(i)? != 'e' {
        let decoded = decode(encoded_value, i)?;
        res.push(decoded.0);
        i = decoded.1;
    }
    Some((serde_json::Value::Array(res), i + 1))
}

fn decode_dict(encoded_value: &str, index: usize) -> Option<(serde_json::Value, usize)> {
    Some((serde_json::Value::String("test".to_string()), 0))
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
