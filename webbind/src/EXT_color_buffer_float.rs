use super::*;

/// The EXT_color_buffer_float class.
/// [`EXT_color_buffer_float`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_color_buffer_float)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_color_buffer_float {
    inner: Any,
}
impl FromVal for EXT_color_buffer_float {
    fn from_val(v: &Any) -> Self {
        EXT_color_buffer_float {
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
impl core::ops::Deref for EXT_color_buffer_float {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_color_buffer_float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EXT_color_buffer_float {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EXT_color_buffer_float {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EXT_color_buffer_float> for Any {
    fn from(s: EXT_color_buffer_float) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EXT_color_buffer_float> for Any {
    fn from(s: &EXT_color_buffer_float) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EXT_color_buffer_float);
