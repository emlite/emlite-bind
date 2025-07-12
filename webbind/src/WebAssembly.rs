use super::*;

pub fn compile_streaming(source: jsbind::Promise) -> jsbind::Promise {
    emlite::Val::global("WebAssembly")
        .call("compileStreaming", &[source.into()])
        .as_::<jsbind::Promise>()
}

pub fn instantiate_streaming0(source: jsbind::Promise) -> jsbind::Promise {
    emlite::Val::global("WebAssembly")
        .call("instantiateStreaming", &[source.into()])
        .as_::<jsbind::Promise>()
}

pub fn instantiate_streaming1(
    source: jsbind::Promise,
    import_object: jsbind::Object,
) -> jsbind::Promise {
    emlite::Val::global("WebAssembly")
        .call(
            "instantiateStreaming",
            &[source.into(), import_object.into()],
        )
        .as_::<jsbind::Promise>()
}
