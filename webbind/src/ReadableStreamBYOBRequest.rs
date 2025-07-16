use super::*;

/// The ReadableStreamBYOBRequest class.
/// [`ReadableStreamBYOBRequest`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamBYOBRequest {
    inner: Any,
}
impl FromVal for ReadableStreamBYOBRequest {
    fn from_val(v: &Any) -> Self {
        ReadableStreamBYOBRequest {
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
impl core::ops::Deref for ReadableStreamBYOBRequest {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamBYOBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableStreamBYOBRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableStreamBYOBRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBRequest> for Any {
    fn from(s: ReadableStreamBYOBRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableStreamBYOBRequest> for Any {
    fn from(s: &ReadableStreamBYOBRequest) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStreamBYOBRequest);

impl ReadableStreamBYOBRequest {
    /// Getter of the `view` attribute.
    /// [`ReadableStreamBYOBRequest.view`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/view)
    pub fn view(&self) -> Any {
        self.inner.get("view").as_::<Any>()
    }
}
impl ReadableStreamBYOBRequest {
    /// The respond method.
    /// [`ReadableStreamBYOBRequest.respond`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respond)
    pub fn respond(&self, bytes_written: u64) -> Undefined {
        self.inner
            .call("respond", &[bytes_written.into()])
            .as_::<Undefined>()
    }
}
impl ReadableStreamBYOBRequest {
    /// The respondWithNewView method.
    /// [`ReadableStreamBYOBRequest.respondWithNewView`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)
    pub fn respond_with_new_view(&self, view: &Any) -> Undefined {
        self.inner
            .call("respondWithNewView", &[view.into()])
            .as_::<Undefined>()
    }
}
