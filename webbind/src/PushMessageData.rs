use super::*;

/// The PushMessageData class.
/// [`PushMessageData`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushMessageData {
    inner: Any,
}
impl FromVal for PushMessageData {
    fn from_val(v: &Any) -> Self {
        PushMessageData {
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
impl core::ops::Deref for PushMessageData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushMessageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PushMessageData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PushMessageData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PushMessageData> for Any {
    fn from(s: PushMessageData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PushMessageData> for Any {
    fn from(s: &PushMessageData) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushMessageData);

impl PushMessageData {
    /// The arrayBuffer method.
    /// [`PushMessageData.arrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/arrayBuffer)
    pub fn array_buffer(&self) -> ArrayBuffer {
        self.inner.call("arrayBuffer", &[]).as_::<ArrayBuffer>()
    }
}
impl PushMessageData {
    /// The blob method.
    /// [`PushMessageData.blob`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/blob)
    pub fn blob(&self) -> Blob {
        self.inner.call("blob", &[]).as_::<Blob>()
    }
}
impl PushMessageData {
    /// The bytes method.
    /// [`PushMessageData.bytes`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/bytes)
    pub fn bytes(&self) -> Uint8Array {
        self.inner.call("bytes", &[]).as_::<Uint8Array>()
    }
}
impl PushMessageData {
    /// The json method.
    /// [`PushMessageData.json`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/json)
    pub fn json(&self) -> Any {
        self.inner.call("json", &[]).as_::<Any>()
    }
}
impl PushMessageData {
    /// The text method.
    /// [`PushMessageData.text`](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/text)
    pub fn text(&self) -> JsString {
        self.inner.call("text", &[]).as_::<JsString>()
    }
}
