use super::*;

/// The RTCOfferAnswerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCOfferAnswerOptions {
    inner: Any,
}

impl FromVal for RTCOfferAnswerOptions {
    fn from_val(v: &Any) -> Self {
        RTCOfferAnswerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCOfferAnswerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCOfferAnswerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCOfferAnswerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCOfferAnswerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCOfferAnswerOptions> for Any {
    fn from(s: RTCOfferAnswerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCOfferAnswerOptions> for Any {
    fn from(s: &RTCOfferAnswerOptions) -> Any {
        s.inner.clone()
    }
}
