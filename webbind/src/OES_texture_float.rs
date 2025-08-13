use super::*;




/// The OES_texture_float class.
/// [`OES_texture_float`](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_float)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_texture_float {
    inner: Any,
}

impl FromVal for OES_texture_float {
    fn from_val(v: &Any) -> Self {
        OES_texture_float { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OES_texture_float {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OES_texture_float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OES_texture_float {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OES_texture_float {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OES_texture_float> for Any {
    fn from(s: OES_texture_float) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OES_texture_float> for Any {
    fn from(s: &OES_texture_float) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OES_texture_float);


