use super::*;

/// The RTCRtpScriptTransform class.
/// [`RTCRtpScriptTransform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpScriptTransform {
    inner: Any,
}

impl FromVal for RTCRtpScriptTransform {
    fn from_val(v: &Any) -> Self {
        RTCRtpScriptTransform {
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

impl core::ops::Deref for RTCRtpScriptTransform {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpScriptTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpScriptTransform {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpScriptTransform {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCRtpScriptTransform> for Any {
    fn from(s: RTCRtpScriptTransform) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpScriptTransform> for Any {
    fn from(s: &RTCRtpScriptTransform) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCRtpScriptTransform);

impl RTCRtpScriptTransform {
    /// The `new RTCRtpScriptTransform(..)` constructor, creating a new RTCRtpScriptTransform instance
    pub fn new0(worker: &Worker) -> RTCRtpScriptTransform {
        Self {
            inner: Any::global("RTCRtpScriptTransform")
                .new(&[worker.into()])
                .as_::<Any>(),
        }
    }

    /// The `new RTCRtpScriptTransform(..)` constructor, creating a new RTCRtpScriptTransform instance
    pub fn new1(worker: &Worker, options: &Any) -> RTCRtpScriptTransform {
        Self {
            inner: Any::global("RTCRtpScriptTransform")
                .new(&[worker.into(), options.into()])
                .as_::<Any>(),
        }
    }

    /// The `new RTCRtpScriptTransform(..)` constructor, creating a new RTCRtpScriptTransform instance
    pub fn new2(
        worker: &Worker,
        options: &Any,
        transfer: &TypedArray<Object>,
    ) -> RTCRtpScriptTransform {
        Self {
            inner: Any::global("RTCRtpScriptTransform")
                .new(&[worker.into(), options.into(), transfer.into()])
                .as_::<Any>(),
        }
    }
}
