use super::*;

/// The RTCRtpScriptTransformer class.
/// [`RTCRtpScriptTransformer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpScriptTransformer {
    inner: EventTarget,
}
impl FromVal for RTCRtpScriptTransformer {
    fn from_val(v: &Any) -> Self {
        RTCRtpScriptTransformer {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpScriptTransformer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpScriptTransformer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpScriptTransformer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpScriptTransformer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpScriptTransformer> for Any {
    fn from(s: RTCRtpScriptTransformer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpScriptTransformer> for Any {
    fn from(s: &RTCRtpScriptTransformer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCRtpScriptTransformer);

impl RTCRtpScriptTransformer {
    /// Getter of the `readable` attribute.
    /// [`RTCRtpScriptTransformer.readable`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl RTCRtpScriptTransformer {
    /// The generateKeyFrame method.
    /// [`RTCRtpScriptTransformer.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/generateKeyFrame)
    pub fn generate_key_frame0(&self) -> Promise<u64> {
        self.inner
            .call("generateKeyFrame", &[])
            .as_::<Promise<u64>>()
    }
    /// The generateKeyFrame method.
    /// [`RTCRtpScriptTransformer.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/generateKeyFrame)
    pub fn generate_key_frame1(&self, rid: &JsString) -> Promise<u64> {
        self.inner
            .call("generateKeyFrame", &[rid.into()])
            .as_::<Promise<u64>>()
    }
}
impl RTCRtpScriptTransformer {
    /// The sendKeyFrameRequest method.
    /// [`RTCRtpScriptTransformer.sendKeyFrameRequest`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/sendKeyFrameRequest)
    pub fn send_key_frame_request(&self) -> Promise<Undefined> {
        self.inner
            .call("sendKeyFrameRequest", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpScriptTransformer {
    /// Getter of the `writable` attribute.
    /// [`RTCRtpScriptTransformer.writable`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl RTCRtpScriptTransformer {
    /// Getter of the `onkeyframerequest` attribute.
    /// [`RTCRtpScriptTransformer.onkeyframerequest`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/onkeyframerequest)
    pub fn onkeyframerequest(&self) -> Any {
        self.inner.get("onkeyframerequest").as_::<Any>()
    }

    /// Setter of the `onkeyframerequest` attribute.
    /// [`RTCRtpScriptTransformer.onkeyframerequest`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/onkeyframerequest)
    pub fn set_onkeyframerequest(&mut self, value: &Any) {
        self.inner.set("onkeyframerequest", value);
    }
}
impl RTCRtpScriptTransformer {
    /// Getter of the `options` attribute.
    /// [`RTCRtpScriptTransformer.options`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/options)
    pub fn options(&self) -> Any {
        self.inner.get("options").as_::<Any>()
    }
}
