use super::*;


/// The validate function from the WebAssembly namespace.
pub fn validate(bytes: &Any) -> bool {
    Any::global("WebAssembly").call("validate", &[bytes.into(), ]).as_::<bool>()
}

/// The compile function from the WebAssembly namespace.
pub fn compile(bytes: &Any) -> Promise<Module> {
    Any::global("WebAssembly").call("compile", &[bytes.into(), ]).as_::<Promise<Module>>()
}

/// The instantiate function from the WebAssembly namespace.
pub fn instantiate0(module_object: &Module) -> Promise<Instance> {
    Any::global("WebAssembly").call("instantiate", &[module_object.into(), ]).as_::<Promise<Instance>>()
}

/// The instantiate function from the WebAssembly namespace.
pub fn instantiate1(module_object: &Module, import_object: &Object) -> Promise<Instance> {
    Any::global("WebAssembly").call("instantiate", &[module_object.into(), import_object.into(), ]).as_::<Promise<Instance>>()
}

/// The compileStreaming function from the WebAssembly namespace.
pub fn compile_streaming(source: &Promise<Response>) -> Promise<Module> {
    Any::global("WebAssembly").call("compileStreaming", &[source.into(), ]).as_::<Promise<Module>>()
}

/// The instantiateStreaming function from the WebAssembly namespace.
pub fn instantiate_streaming0(source: &Promise<Response>) -> Promise<WebAssemblyInstantiatedSource> {
    Any::global("WebAssembly").call("instantiateStreaming", &[source.into(), ]).as_::<Promise<WebAssemblyInstantiatedSource>>()
}

/// The instantiateStreaming function from the WebAssembly namespace.
pub fn instantiate_streaming1(source: &Promise<Response>, import_object: &Object) -> Promise<WebAssemblyInstantiatedSource> {
    Any::global("WebAssembly").call("instantiateStreaming", &[source.into(), import_object.into(), ]).as_::<Promise<WebAssemblyInstantiatedSource>>()
}

