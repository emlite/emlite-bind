use super::*;

/// The XRWebGLDepthInformation class.
/// [`XRWebGLDepthInformation`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLDepthInformation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLDepthInformation {
    inner: XRDepthInformation,
}

impl FromVal for XRWebGLDepthInformation {
    fn from_val(v: &Any) -> Self {
        XRWebGLDepthInformation {
            inner: XRDepthInformation::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRWebGLDepthInformation {
    type Target = XRDepthInformation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRWebGLDepthInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRWebGLDepthInformation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRWebGLDepthInformation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRWebGLDepthInformation> for Any {
    fn from(s: XRWebGLDepthInformation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRWebGLDepthInformation> for Any {
    fn from(s: &XRWebGLDepthInformation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRWebGLDepthInformation);

impl XRWebGLDepthInformation {
    /// Getter of the `texture` attribute.
    /// [`XRWebGLDepthInformation.texture`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLDepthInformation/texture)
    pub fn texture(&self) -> WebGLTexture {
        self.inner.get("texture").as_::<WebGLTexture>()
    }
}
impl XRWebGLDepthInformation {
    /// Getter of the `textureType` attribute.
    /// [`XRWebGLDepthInformation.textureType`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLDepthInformation/textureType)
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }
}
impl XRWebGLDepthInformation {
    /// Getter of the `imageIndex` attribute.
    /// [`XRWebGLDepthInformation.imageIndex`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLDepthInformation/imageIndex)
    pub fn image_index(&self) -> u32 {
        self.inner.get("imageIndex").as_::<u32>()
    }
}
