use super::*;

/// The EXT_texture_filter_anisotropic class.
/// [`EXT_texture_filter_anisotropic`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_filter_anisotropic)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_texture_filter_anisotropic {
    inner: Any,
}

impl FromVal for EXT_texture_filter_anisotropic {
    fn from_val(v: &Any) -> Self {
        EXT_texture_filter_anisotropic {
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

impl core::ops::Deref for EXT_texture_filter_anisotropic {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_texture_filter_anisotropic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_texture_filter_anisotropic {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_texture_filter_anisotropic {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EXT_texture_filter_anisotropic> for Any {
    fn from(s: EXT_texture_filter_anisotropic) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_texture_filter_anisotropic> for Any {
    fn from(s: &EXT_texture_filter_anisotropic) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_texture_filter_anisotropic);
