use super::*;

/// The XPathResult class.
/// [`XPathResult`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathResult {
    inner: Any,
}

impl FromVal for XPathResult {
    fn from_val(v: &Any) -> Self {
        XPathResult {
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

impl core::ops::Deref for XPathResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XPathResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XPathResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XPathResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XPathResult> for Any {
    fn from(s: XPathResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XPathResult> for Any {
    fn from(s: &XPathResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XPathResult);

impl XPathResult {
    /// Getter of the `resultType` attribute.
    /// [`XPathResult.resultType`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/resultType)
    pub fn result_type(&self) -> u16 {
        self.inner.get("resultType").as_::<u16>()
    }
}
impl XPathResult {
    /// Getter of the `numberValue` attribute.
    /// [`XPathResult.numberValue`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/numberValue)
    pub fn number_value(&self) -> f64 {
        self.inner.get("numberValue").as_::<f64>()
    }
}
impl XPathResult {
    /// Getter of the `stringValue` attribute.
    /// [`XPathResult.stringValue`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/stringValue)
    pub fn string_value(&self) -> JsString {
        self.inner.get("stringValue").as_::<JsString>()
    }
}
impl XPathResult {
    /// Getter of the `booleanValue` attribute.
    /// [`XPathResult.booleanValue`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/booleanValue)
    pub fn boolean_value(&self) -> bool {
        self.inner.get("booleanValue").as_::<bool>()
    }
}
impl XPathResult {
    /// Getter of the `singleNodeValue` attribute.
    /// [`XPathResult.singleNodeValue`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/singleNodeValue)
    pub fn single_node_value(&self) -> Node {
        self.inner.get("singleNodeValue").as_::<Node>()
    }
}
impl XPathResult {
    /// Getter of the `invalidIteratorState` attribute.
    /// [`XPathResult.invalidIteratorState`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/invalidIteratorState)
    pub fn invalid_iterator_state(&self) -> bool {
        self.inner.get("invalidIteratorState").as_::<bool>()
    }
}
impl XPathResult {
    /// Getter of the `snapshotLength` attribute.
    /// [`XPathResult.snapshotLength`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotLength)
    pub fn snapshot_length(&self) -> u32 {
        self.inner.get("snapshotLength").as_::<u32>()
    }
}
impl XPathResult {
    /// The iterateNext method.
    /// [`XPathResult.iterateNext`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/iterateNext)
    pub fn iterate_next(&self) -> Node {
        self.inner.call("iterateNext", &[]).as_::<Node>()
    }
}
impl XPathResult {
    /// The snapshotItem method.
    /// [`XPathResult.snapshotItem`](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotItem)
    pub fn snapshot_item(&self, index: u32) -> Node {
        self.inner
            .call("snapshotItem", &[index.into()])
            .as_::<Node>()
    }
}
