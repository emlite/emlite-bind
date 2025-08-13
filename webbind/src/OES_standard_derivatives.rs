use super::*;




/// The OES_standard_derivatives class.
/// [`OES_standard_derivatives`](https://developer.mozilla.org/en-US/docs/Web/API/OES_standard_derivatives)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_standard_derivatives {
    inner: Any,
}

impl FromVal for OES_standard_derivatives {
    fn from_val(v: &Any) -> Self {
        OES_standard_derivatives { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OES_standard_derivatives {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OES_standard_derivatives {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OES_standard_derivatives {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OES_standard_derivatives {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OES_standard_derivatives> for Any {
    fn from(s: OES_standard_derivatives) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OES_standard_derivatives> for Any {
    fn from(s: &OES_standard_derivatives) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OES_standard_derivatives);


