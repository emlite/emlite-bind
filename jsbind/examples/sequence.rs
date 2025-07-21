//! Example usage of jsbind::Sequence

use jsbind::prelude::*;

fn main() {
    // Create a Sequence from a Vec of Any
    let seq = Sequence::from(vec![Any::from(1), Any::from(2), Any::from(3)]);
    println!("Created Sequence: {:?}", seq);

    // Access elements
    for (i, val) in seq.iter().enumerate() {
        println!("Element {}: {:?}", i, val);
    }

    // Convert Sequence back to Vec<Any> (if supported by your API)
    // let vec: Vec<Any> = seq.clone().into();
    // println!("Converted back to Vec: {:?}", vec);
    // If not supported, just print the sequence
    println!("Sequence: {:?}", seq);
}
