use super::*;

pub fn compile_streaming(source: &Promise<Response>) -> Promise<Module> {
    Any::global("WebAssembly")
        .call("compileStreaming", &[source.into()])
        .as_::<Promise<Module>>()
}

pub fn instantiate_streaming0(source: &Promise<Response>) -> Promise<Any> {
    Any::global("WebAssembly")
        .call("instantiateStreaming", &[source.into()])
        .as_::<Promise<Any>>()
}

pub fn instantiate_streaming1(source: &Promise<Response>, import_object: &Object) -> Promise<Any> {
    Any::global("WebAssembly")
        .call(
            "instantiateStreaming",
            &[source.into(), import_object.into()],
        )
        .as_::<Promise<Any>>()
}
