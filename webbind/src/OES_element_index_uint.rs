use super::*;




/// The OES_element_index_uint class.
/// [`OES_element_index_uint`](https://developer.mozilla.org/en-US/docs/Web/API/OES_element_index_uint)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_element_index_uint {
    inner: Any,
}

impl FromVal for OES_element_index_uint {
    fn from_val(v: &Any) -> Self {
        OES_element_index_uint { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OES_element_index_uint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OES_element_index_uint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OES_element_index_uint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OES_element_index_uint {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OES_element_index_uint> for Any {
    fn from(s: OES_element_index_uint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OES_element_index_uint> for Any {
    fn from(s: &OES_element_index_uint) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OES_element_index_uint);


