extern crate serde;
extern crate serde_cbor;

use std::collections::HashMap;

use serde_cbor::{Value, to_vec, to_vec_sd, from_slice};

#[test]
fn test_string() {
    let value = "foobar".to_owned();
    assert_eq!(&to_vec(&value).unwrap()[..], b"ffoobar");
}

#[test]
fn test_list() {
    let value = vec![1, 2, 3];
    assert_eq!(&to_vec(&value).unwrap()[..], b"\x83\x01\x02\x03");
}

#[test]
fn test_object() {
    let mut object = HashMap::new();
    object.insert("a".to_owned(), "A".to_owned());
    object.insert("b".to_owned(), "B".to_owned());
    object.insert("c".to_owned(), "C".to_owned());
    object.insert("d".to_owned(), "D".to_owned());
    object.insert("e".to_owned(), "E".to_owned());
    let vec = to_vec(&object).unwrap();
    let test_object = from_slice(&vec[..]).unwrap();
    assert_eq!(object, test_object);
}

#[test]
fn test_float() {
    let vec = to_vec(&12.3f64).unwrap();
    assert_eq!(vec, b"\xfb@(\x99\x99\x99\x99\x99\x9a");
}

#[test]
fn test_f32() {
    let vec = to_vec(&4000.5f32).unwrap();
    assert_eq!(vec, b"\xfa\x45\x7a\x08\x00");
}

#[test]
fn test_infinity() {
    let vec = to_vec(&::std::f64::INFINITY).unwrap();
    assert_eq!(vec, b"\xf9|\x00");
}

#[test]
fn test_neg_infinity() {
    let vec = to_vec(&::std::f64::NEG_INFINITY).unwrap();
    assert_eq!(vec, b"\xf9\xfc\x00");
}

#[test]
fn test_nan() {
    let vec = to_vec(&::std::f32::NAN).unwrap();
    assert_eq!(vec, b"\xf9\x7e\x00");
}

#[test]
fn test_integer() {
    // u8
    let vec = to_vec(&24).unwrap();
    assert_eq!(vec, b"\x18\x18");
    // i8
    let vec = to_vec(&-5).unwrap();
    assert_eq!(vec, b"\x24");
    // i16
    let vec = to_vec(&-300).unwrap();
    assert_eq!(vec, b"\x39\x01\x2b");
    // i32
    let vec = to_vec(&-23567997).unwrap();
    assert_eq!(vec, b"\x3a\x01\x67\x9e\x7c");
    // u64
    let vec = to_vec(&::std::u64::MAX).unwrap();
    assert_eq!(vec, b"\x1b\xff\xff\xff\xff\xff\xff\xff\xff");
}

#[test]
fn test_self_describing() {
    let vec = to_vec_sd(&0).unwrap();
    assert_eq!(vec, b"\xd9\xd9\xf7\x00");
}

#[test]
fn test_tagged_value() {
    let value = Value::Tag(0, Box::new(Value::String("2016-06-23T11:38:01Z".to_owned())));
    let vec = to_vec(&value).unwrap();
    assert_eq!(vec, b"\xc0\x742016-06-23T11:38:01Z");
    let roundtrip = from_slice(&vec[..]).unwrap();
    assert_eq!(value, roundtrip);
}
