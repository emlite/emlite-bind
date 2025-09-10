use super::*;

/// The WEBGL_draw_buffers class.
/// [`WEBGL_draw_buffers`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_draw_buffers {
    inner: Any,
}

impl FromVal for WEBGL_draw_buffers {
    fn from_val(v: &Any) -> Self {
        WEBGL_draw_buffers {
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

impl core::ops::Deref for WEBGL_draw_buffers {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_draw_buffers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_draw_buffers {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_draw_buffers {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_draw_buffers> for Any {
    fn from(s: WEBGL_draw_buffers) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_draw_buffers> for Any {
    fn from(s: &WEBGL_draw_buffers) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_draw_buffers);

impl WEBGL_draw_buffers {
    /// The drawBuffersWEBGL method.
    /// [`WEBGL_draw_buffers.drawBuffersWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers/drawBuffersWEBGL)
    pub fn draw_buffers_webgl(&self, buffers: &TypedArray<Any>) -> Undefined {
        self.inner
            .call("drawBuffersWEBGL", &[buffers.into()])
            .as_::<Undefined>()
    }
}
