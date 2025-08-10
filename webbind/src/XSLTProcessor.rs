use super::*;

/// The XSLTProcessor class.
/// [`XSLTProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XSLTProcessor {
    inner: Any,
}

impl FromVal for XSLTProcessor {
    fn from_val(v: &Any) -> Self {
        XSLTProcessor {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XSLTProcessor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XSLTProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XSLTProcessor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XSLTProcessor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XSLTProcessor> for Any {
    fn from(s: XSLTProcessor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XSLTProcessor> for Any {
    fn from(s: &XSLTProcessor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XSLTProcessor);

impl XSLTProcessor {
    /// The `new XSLTProcessor(..)` constructor, creating a new XSLTProcessor instance
    pub fn new() -> XSLTProcessor {
        Self {
            inner: Any::global("XSLTProcessor").new(&[]).as_::<Any>(),
        }
    }
}
impl XSLTProcessor {
    /// The importStylesheet method.
    /// [`XSLTProcessor.importStylesheet`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/importStylesheet)
    pub fn import_stylesheet(&self, style: &Node) -> Undefined {
        self.inner
            .call("importStylesheet", &[style.into()])
            .as_::<Undefined>()
    }
}
impl XSLTProcessor {
    /// The transformToFragment method.
    /// [`XSLTProcessor.transformToFragment`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToFragment)
    pub fn transform_to_fragment(&self, source: &Node, output: &Document) -> DocumentFragment {
        self.inner
            .call("transformToFragment", &[source.into(), output.into()])
            .as_::<DocumentFragment>()
    }
}
impl XSLTProcessor {
    /// The transformToDocument method.
    /// [`XSLTProcessor.transformToDocument`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToDocument)
    pub fn transform_to_document(&self, source: &Node) -> Document {
        self.inner
            .call("transformToDocument", &[source.into()])
            .as_::<Document>()
    }
}
impl XSLTProcessor {
    /// The setParameter method.
    /// [`XSLTProcessor.setParameter`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/setParameter)
    pub fn set_parameter(
        &self,
        namespace_uri: &JsString,
        local_name: &JsString,
        value: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "setParameter",
                &[namespace_uri.into(), local_name.into(), value.into()],
            )
            .as_::<Undefined>()
    }
}
impl XSLTProcessor {
    /// The getParameter method.
    /// [`XSLTProcessor.getParameter`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/getParameter)
    pub fn get_parameter(&self, namespace_uri: &JsString, local_name: &JsString) -> Any {
        self.inner
            .call("getParameter", &[namespace_uri.into(), local_name.into()])
            .as_::<Any>()
    }
}
impl XSLTProcessor {
    /// The removeParameter method.
    /// [`XSLTProcessor.removeParameter`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/removeParameter)
    pub fn remove_parameter(&self, namespace_uri: &JsString, local_name: &JsString) -> Undefined {
        self.inner
            .call(
                "removeParameter",
                &[namespace_uri.into(), local_name.into()],
            )
            .as_::<Undefined>()
    }
}
impl XSLTProcessor {
    /// The clearParameters method.
    /// [`XSLTProcessor.clearParameters`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/clearParameters)
    pub fn clear_parameters(&self) -> Undefined {
        self.inner.call("clearParameters", &[]).as_::<Undefined>()
    }
}
impl XSLTProcessor {
    /// The reset method.
    /// [`XSLTProcessor.reset`](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
