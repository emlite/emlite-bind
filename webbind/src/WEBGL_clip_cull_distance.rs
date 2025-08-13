use super::*;




/// The WEBGL_clip_cull_distance class.
/// [`WEBGL_clip_cull_distance`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_clip_cull_distance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_clip_cull_distance {
    inner: Any,
}

impl FromVal for WEBGL_clip_cull_distance {
    fn from_val(v: &Any) -> Self {
        WEBGL_clip_cull_distance { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WEBGL_clip_cull_distance {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_clip_cull_distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_clip_cull_distance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_clip_cull_distance {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WEBGL_clip_cull_distance> for Any {
    fn from(s: WEBGL_clip_cull_distance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_clip_cull_distance> for Any {
    fn from(s: &WEBGL_clip_cull_distance) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_clip_cull_distance);


