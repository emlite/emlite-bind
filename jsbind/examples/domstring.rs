use jsbind::prelude::*;

fn main() {
    emlite::init();
    // UTF-8 string example
    let s: JsString = "Hello".into();
    Console::get().log(&[s.clone().into()]);
    println!("UTF-8: {:?}", s.as_string());

    // UTF-16 string example
    let utf16_data: Vec<u16> = "Hello 世界".encode_utf16().collect();
    let s_utf16 = JsString::from_utf16(&utf16_data);
    Console::get().log(&[s_utf16.clone().into()]);
    println!("UTF-16 length: {}", s_utf16.length());

    // Extract UTF-16 data back
    if let Some(extracted_utf16) = s_utf16.to_utf16() {
        println!("Extracted UTF-16: {:?}", extracted_utf16);
        if let Ok(rust_string) = JsString::utf16_to_string(&extracted_utf16) {
            println!("Converted back to Rust String: {:?}", rust_string);
        }
    }

    // Alternative UTF-16 creation method
    let s_via_utf16 = JsString::from_string_via_utf16("🚀 Rust + JavaScript");
    Console::get().log(&[s_via_utf16.clone().into()]);

    // Direct conversion from UTF-16 to Rust String
    if let Some(converted) = s_via_utf16.to_string_from_utf16() {
        println!("Direct UTF-16 -> String conversion: {:?}", converted);
    }
}
