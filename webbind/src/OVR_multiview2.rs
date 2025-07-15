use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for OVR_multiview2 {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OVR_multiview2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OVR_multiview2 {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OVR_multiview2 {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OVR_multiview2> for emlite::Val {
    fn from(s: OVR_multiview2) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&OVR_multiview2> for emlite::Val {
    fn from(s: &OVR_multiview2) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OVR_multiview2);

impl OVR_multiview2 {
    pub fn framebuffer_texture_multiview_ovr(
        &self,
        target: Any,
        attachment: Any,
        texture: WebGLTexture,
        level: Any,
        base_view_index: Any,
        num_views: Any,
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
