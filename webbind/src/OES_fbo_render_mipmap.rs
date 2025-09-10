use super::*;

/// The OES_fbo_render_mipmap class.
/// [`OES_fbo_render_mipmap`](https://developer.mozilla.org/en-US/docs/Web/API/OES_fbo_render_mipmap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_fbo_render_mipmap {
    inner: Any,
}

impl FromVal for OES_fbo_render_mipmap {
    fn from_val(v: &Any) -> Self {
        OES_fbo_render_mipmap {
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

impl core::ops::Deref for OES_fbo_render_mipmap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OES_fbo_render_mipmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OES_fbo_render_mipmap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OES_fbo_render_mipmap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<OES_fbo_render_mipmap> for Any {
    fn from(s: OES_fbo_render_mipmap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OES_fbo_render_mipmap> for Any {
    fn from(s: &OES_fbo_render_mipmap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OES_fbo_render_mipmap);
