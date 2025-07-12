use super::*;

#[derive(Clone, Debug)]
pub struct XRWebGLDepthInformation {
    inner: XRDepthInformation,
}
impl FromVal for XRWebGLDepthInformation {
    fn from_val(v: &emlite::Val) -> Self {
        XRWebGLDepthInformation {
            inner: XRDepthInformation::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRWebGLDepthInformation {
    type Target = XRDepthInformation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRWebGLDepthInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRWebGLDepthInformation> for emlite::Val {
    fn from(s: XRWebGLDepthInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRWebGLDepthInformation {
    pub fn texture(&self) -> WebGLTexture {
        self.inner.get("texture").as_::<WebGLTexture>()
    }
}
impl XRWebGLDepthInformation {
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }
}
impl XRWebGLDepthInformation {
    pub fn image_index(&self) -> u32 {
        self.inner.get("imageIndex").as_::<u32>()
    }
}
