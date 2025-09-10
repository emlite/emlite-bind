use super::*;

/// The EXT_texture_compression_bptc class.
/// [`EXT_texture_compression_bptc`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_compression_bptc)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_texture_compression_bptc {
    inner: Any,
}

impl FromVal for EXT_texture_compression_bptc {
    fn from_val(v: &Any) -> Self {
        EXT_texture_compression_bptc {
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

impl core::ops::Deref for EXT_texture_compression_bptc {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_texture_compression_bptc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_texture_compression_bptc {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_texture_compression_bptc {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EXT_texture_compression_bptc> for Any {
    fn from(s: EXT_texture_compression_bptc) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_texture_compression_bptc> for Any {
    fn from(s: &EXT_texture_compression_bptc) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_texture_compression_bptc);
