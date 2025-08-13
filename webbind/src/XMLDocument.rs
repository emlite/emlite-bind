use super::*;




/// The XMLDocument class.
/// [`XMLDocument`](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLDocument {
    inner: Document,
}

impl FromVal for XMLDocument {
    fn from_val(v: &Any) -> Self {
        XMLDocument { inner: Document::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XMLDocument {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XMLDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XMLDocument {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XMLDocument {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XMLDocument> for Any {
    fn from(s: XMLDocument) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XMLDocument> for Any {
    fn from(s: &XMLDocument) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XMLDocument);


