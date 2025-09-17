use super::*;

/// Callback interface XPathNSResolver
/// Generated wrapper for WebIDL `callback interface XPathNSResolver`
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathNSResolver {
    inner: Any,
}

impl FromVal for XPathNSResolver {
    fn from_val(v: &Any) -> Self {
        XPathNSResolver { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        XPathNSResolver {
            inner: Any::take_ownership(v),
        }
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XPathNSResolver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XPathNSResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<XPathNSResolver> for Any {
    fn from(s: XPathNSResolver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XPathNSResolver> for Any {
    fn from(s: &XPathNSResolver) -> Any {
        s.inner.clone().into()
    }
}

impl XPathNSResolver {
    /// Wrap a JavaScript function as a XPathNSResolver
    pub fn from_function(f: &Function) -> XPathNSResolver {
        XPathNSResolver {
            inner: Any::from(f.clone()),
        }
    }
}

impl XPathNSResolver {
    /// Build a XPathNSResolver from a typed Rust closure matching `lookupNamespaceURI`
    pub fn from_closure<F>(mut cb: F) -> XPathNSResolver
    where
        F: FnMut(JsString) -> JsString + 'static,
    {
        let c = Closure::bind1(move |a1: JsString| cb(a1));
        XPathNSResolver::from_function(&Function::from(&c))
    }
}

impl XPathNSResolver {
    pub fn lookup_namespace_uri(&self, prefix: &JsString) -> JsString {
        if self.inner.is_function() {
            // Call as a bare function
            self.inner.invoke(&[prefix.into()]).as_::<JsString>()
        } else {
            // Call the named method on the object
            self.inner
                .call("lookupNamespaceURI", &[prefix.into()])
                .as_::<JsString>()
        }
    }
}
