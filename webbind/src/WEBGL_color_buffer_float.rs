use super::*;

/// The WEBGL_color_buffer_float class.
/// [`WEBGL_color_buffer_float`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_color_buffer_float)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_color_buffer_float {
    inner: Any,
}
impl FromVal for WEBGL_color_buffer_float {
    fn from_val(v: &Any) -> Self {
        WEBGL_color_buffer_float {
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
impl core::ops::Deref for WEBGL_color_buffer_float {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_color_buffer_float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WEBGL_color_buffer_float {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WEBGL_color_buffer_float {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WEBGL_color_buffer_float> for Any {
    fn from(s: WEBGL_color_buffer_float) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WEBGL_color_buffer_float> for Any {
    fn from(s: &WEBGL_color_buffer_float) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_color_buffer_float);
