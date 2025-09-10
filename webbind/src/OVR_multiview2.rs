use super::*;

/// The OVR_multiview2 class.
/// [`OVR_multiview2`](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OVR_multiview2 {
    inner: Any,
}

impl FromVal for OVR_multiview2 {
    fn from_val(v: &Any) -> Self {
        OVR_multiview2 {
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

impl core::ops::Deref for OVR_multiview2 {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OVR_multiview2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OVR_multiview2 {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OVR_multiview2 {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<OVR_multiview2> for Any {
    fn from(s: OVR_multiview2) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OVR_multiview2> for Any {
    fn from(s: &OVR_multiview2) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OVR_multiview2);

impl OVR_multiview2 {
    /// The framebufferTextureMultiviewOVR method.
    /// [`OVR_multiview2.framebufferTextureMultiviewOVR`](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2/framebufferTextureMultiviewOVR)
    pub fn framebuffer_texture_multiview_ovr(
        &self,
        target: &Any,
        attachment: &Any,
        texture: &WebGLTexture,
        level: &Any,
        base_view_index: &Any,
        num_views: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "framebufferTextureMultiviewOVR",
                &[
                    target.into(),
                    attachment.into(),
                    texture.into(),
                    level.into(),
                    base_view_index.into(),
                    num_views.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
