use super::*;

/// The GPUCompilationMessage class.
/// [`GPUCompilationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCompilationMessage {
    inner: Any,
}
impl FromVal for GPUCompilationMessage {
    fn from_val(v: &Any) -> Self {
        GPUCompilationMessage {
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
impl core::ops::Deref for GPUCompilationMessage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCompilationMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCompilationMessage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCompilationMessage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCompilationMessage> for Any {
    fn from(s: GPUCompilationMessage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCompilationMessage> for Any {
    fn from(s: &GPUCompilationMessage) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUCompilationMessage);

impl GPUCompilationMessage {
    /// Getter of the `message` attribute.
    /// [`GPUCompilationMessage.message`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }
}
impl GPUCompilationMessage {
    /// Getter of the `type` attribute.
    /// [`GPUCompilationMessage.type`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/type)
    pub fn type_(&self) -> GPUCompilationMessageType {
        self.inner.get("type").as_::<GPUCompilationMessageType>()
    }
}
impl GPUCompilationMessage {
    /// Getter of the `lineNum` attribute.
    /// [`GPUCompilationMessage.lineNum`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/lineNum)
    pub fn line_num(&self) -> u64 {
        self.inner.get("lineNum").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    /// Getter of the `linePos` attribute.
    /// [`GPUCompilationMessage.linePos`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/linePos)
    pub fn line_pos(&self) -> u64 {
        self.inner.get("linePos").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    /// Getter of the `offset` attribute.
    /// [`GPUCompilationMessage.offset`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/offset)
    pub fn offset(&self) -> u64 {
        self.inner.get("offset").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    /// Getter of the `length` attribute.
    /// [`GPUCompilationMessage.length`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/length)
    pub fn length(&self) -> u64 {
        self.inner.get("length").as_::<u64>()
    }
}
