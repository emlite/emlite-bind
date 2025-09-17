#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::too_many_arguments)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use jsbind::prelude::*;

#[allow(unused_macros)]
macro_rules! web_feature {
    ($feature:literal, $mod_ident:ident, $path:literal, $type_ident:ident) => {
        #[cfg(feature = $feature)]
        #[path = $path]
        pub mod $mod_ident;
        #[cfg(feature = $feature)]
        pub use $mod_ident::*;

        #[cfg(not(feature = $feature))]
        pub type $type_ident = Any;
    };
}

include!(concat!(env!("OUT_DIR"), "/generated_mods.rs"));

pub fn window() -> Window {
    Any::global("window").as_::<Window>()
}

pub use jsbind::prelude::atob;
pub use jsbind::prelude::btoa;
pub use jsbind::prelude::is_nan;
pub use jsbind::prelude::parse_float;
pub use jsbind::prelude::parse_int;
pub use jsbind::prelude::queue_microtask;
pub use jsbind::prelude::{structured_clone, JsStructuredSerializeOptions};

pub fn report_error(error: &jsbind::prelude::JsError) {
    Any::global("reportError").invoke(&[error.into()]);
}

pub fn caches() -> CacheStorage {
    Any::global("caches").as_()
}

pub fn cross_origin_isolated() -> bool {
    Any::global("crossOriginIsolated").as_::<bool>()
}

pub fn crypto() -> Crypto {
    Any::global("crypto").as_()
}

pub fn indexed_db() -> IDBFactory {
    Any::global("indexedDB").as_()
}

pub fn is_secure_context() -> bool {
    Any::global("isSecureContext").as_()
}

pub fn origin() -> jsbind::prelude::JsString {
    Any::global("origin").as_()
}

// #[cfg(feature = "Performance")]
pub fn performance() -> self::performance::Performance {
    Any::global("performance").as_()
}

pub fn create_image_bitmap0(
    image: &jsbind::prelude::Any,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone()])
        .as_()
}

pub fn create_image_bitmap1(
    image: &jsbind::prelude::Any,
    options: &jsbind::prelude::Object,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone(), options.into()])
        .as_()
}

pub fn create_image_bitmap2(
    image: &jsbind::prelude::Any,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone(), sx.into(), sy.into(), sw.into(), sh.into()])
        .as_()
}

pub fn create_image_bitmap3(
    image: &jsbind::prelude::Any,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    options: &jsbind::prelude::Object,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[
            image.clone(),
            sx.into(),
            sy.into(),
            sw.into(),
            sh.into(),
            options.into(),
        ])
        .as_()
}
