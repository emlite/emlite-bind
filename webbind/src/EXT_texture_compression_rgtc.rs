use super::*;

/// The EXT_texture_compression_rgtc class.
/// [`EXT_texture_compression_rgtc`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_compression_rgtc)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_texture_compression_rgtc {
    inner: Any,
}

impl FromVal for EXT_texture_compression_rgtc {
    fn from_val(v: &Any) -> Self {
        EXT_texture_compression_rgtc {
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

impl core::ops::Deref for EXT_texture_compression_rgtc {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_texture_compression_rgtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_texture_compression_rgtc {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_texture_compression_rgtc {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EXT_texture_compression_rgtc> for Any {
    fn from(s: EXT_texture_compression_rgtc) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_texture_compression_rgtc> for Any {
    fn from(s: &EXT_texture_compression_rgtc) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_texture_compression_rgtc);
