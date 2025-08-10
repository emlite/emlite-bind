use super::*;

/// The WEBGL_compressed_texture_pvrtc class.
/// [`WEBGL_compressed_texture_pvrtc`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_pvrtc)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_compressed_texture_pvrtc {
    inner: Any,
}

impl FromVal for WEBGL_compressed_texture_pvrtc {
    fn from_val(v: &Any) -> Self {
        WEBGL_compressed_texture_pvrtc {
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

impl core::ops::Deref for WEBGL_compressed_texture_pvrtc {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_compressed_texture_pvrtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_compressed_texture_pvrtc {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_compressed_texture_pvrtc {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_compressed_texture_pvrtc> for Any {
    fn from(s: WEBGL_compressed_texture_pvrtc) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_compressed_texture_pvrtc> for Any {
    fn from(s: &WEBGL_compressed_texture_pvrtc) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_compressed_texture_pvrtc);
