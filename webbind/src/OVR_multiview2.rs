use super::*;

#[derive(Clone, Debug)]
pub struct OVR_multiview2 {
    inner: emlite::Val,
}
impl FromVal for OVR_multiview2 {
    fn from_val(v: &emlite::Val) -> Self {
        OVR_multiview2 {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OVR_multiview2 {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OVR_multiview2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OVR_multiview2> for emlite::Val {
    fn from(s: OVR_multiview2) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OVR_multiview2 {
    pub fn framebuffer_texture_multiview_ovr(
        &self,
        target: jsbind::Any,
        attachment: jsbind::Any,
        texture: WebGLTexture,
        level: jsbind::Any,
        base_view_index: jsbind::Any,
        num_views: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
