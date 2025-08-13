use super::*;




/// The EXT_blend_minmax class.
/// [`EXT_blend_minmax`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_blend_minmax)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_blend_minmax {
    inner: Any,
}

impl FromVal for EXT_blend_minmax {
    fn from_val(v: &Any) -> Self {
        EXT_blend_minmax { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EXT_blend_minmax {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_blend_minmax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_blend_minmax {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_blend_minmax {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EXT_blend_minmax> for Any {
    fn from(s: EXT_blend_minmax) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_blend_minmax> for Any {
    fn from(s: &EXT_blend_minmax) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_blend_minmax);


