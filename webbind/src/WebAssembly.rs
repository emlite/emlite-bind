use super::*;

pub fn compile_streaming(source: Promise) -> Promise {
    emlite::Val::global("WebAssembly")
        .call("compileStreaming", &[source.into()])
        .as_::<Promise>()
}

pub fn instantiate_streaming0(source: Promise) -> Promise {
    emlite::Val::global("WebAssembly")
        .call("instantiateStreaming", &[source.into()])
        .as_::<Promise>()
}

pub fn instantiate_streaming1(source: Promise, import_object: Object) -> Promise {
    emlite::Val::global("WebAssembly")
        .call(
            "instantiateStreaming",
            &[source.into(), import_object.into()],
        )
        .as_::<Promise>()
}
