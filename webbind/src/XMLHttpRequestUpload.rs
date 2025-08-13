use super::*;




/// The XMLHttpRequestUpload class.
/// [`XMLHttpRequestUpload`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestUpload)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLHttpRequestUpload {
    inner: XMLHttpRequestEventTarget,
}

impl FromVal for XMLHttpRequestUpload {
    fn from_val(v: &Any) -> Self {
        XMLHttpRequestUpload { inner: XMLHttpRequestEventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XMLHttpRequestUpload {
    type Target = XMLHttpRequestEventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XMLHttpRequestUpload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XMLHttpRequestUpload {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XMLHttpRequestUpload {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XMLHttpRequestUpload> for Any {
    fn from(s: XMLHttpRequestUpload) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XMLHttpRequestUpload> for Any {
    fn from(s: &XMLHttpRequestUpload) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XMLHttpRequestUpload);


