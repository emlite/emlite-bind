//! Example usage of jsbind::TypedArray

use jsbind::prelude::*;

fn main() {
    emlite::init();
    // Create a TypedArray from a Vec of Any
    let seq = TypedArray::from(vec![Any::from(1), Any::from(2), Any::from(3)]);
    println!("Created TypedArray: {:?}", seq);

    // Access elements
    for (i, val) in seq.iter().enumerate() {
        println!("Element {}: {:?}", i, val);
    }

    // Convert TypedArray back to Vec<Any> (if supported by your API)
    // let vec: Vec<Any> = seq.clone().into();
    // println!("Converted back to Vec: {:?}", vec);
    // If not supported, just print the sequence
    println!("TypedArray: {:?}", seq);
}
