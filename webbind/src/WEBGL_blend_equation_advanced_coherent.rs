use super::*;

/// The WEBGL_blend_equation_advanced_coherent class.
/// [`WEBGL_blend_equation_advanced_coherent`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_blend_equation_advanced_coherent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_blend_equation_advanced_coherent {
    inner: Any,
}

impl FromVal for WEBGL_blend_equation_advanced_coherent {
    fn from_val(v: &Any) -> Self {
        WEBGL_blend_equation_advanced_coherent {
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

impl core::ops::Deref for WEBGL_blend_equation_advanced_coherent {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_blend_equation_advanced_coherent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_blend_equation_advanced_coherent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_blend_equation_advanced_coherent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_blend_equation_advanced_coherent> for Any {
    fn from(s: WEBGL_blend_equation_advanced_coherent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_blend_equation_advanced_coherent> for Any {
    fn from(s: &WEBGL_blend_equation_advanced_coherent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_blend_equation_advanced_coherent);
