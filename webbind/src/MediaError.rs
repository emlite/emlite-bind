use super::*;

/// The MediaError class.
/// [`MediaError`](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaError {
    inner: Any,
}

impl FromVal for MediaError {
    fn from_val(v: &Any) -> Self {
        MediaError {
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

impl core::ops::Deref for MediaError {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaError> for Any {
    fn from(s: MediaError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaError> for Any {
    fn from(s: &MediaError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaError);

impl MediaError {
    /// Getter of the `code` attribute.
    /// [`MediaError.code`](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
impl MediaError {
    /// Getter of the `message` attribute.
    /// [`MediaError.message`](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }
}
