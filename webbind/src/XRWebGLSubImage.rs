use super::*;

/// The XRWebGLSubImage class.
/// [`XRWebGLSubImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLSubImage {
    inner: XRSubImage,
}

impl FromVal for XRWebGLSubImage {
    fn from_val(v: &Any) -> Self {
        XRWebGLSubImage {
            inner: XRSubImage::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRWebGLSubImage {
    type Target = XRSubImage;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRWebGLSubImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRWebGLSubImage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRWebGLSubImage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRWebGLSubImage> for Any {
    fn from(s: XRWebGLSubImage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRWebGLSubImage> for Any {
    fn from(s: &XRWebGLSubImage) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRWebGLSubImage);

impl XRWebGLSubImage {
    /// Getter of the `colorTexture` attribute.
    /// [`XRWebGLSubImage.colorTexture`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/colorTexture)
    pub fn color_texture(&self) -> WebGLTexture {
        self.inner.get("colorTexture").as_::<WebGLTexture>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `depthStencilTexture` attribute.
    /// [`XRWebGLSubImage.depthStencilTexture`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/depthStencilTexture)
    pub fn depth_stencil_texture(&self) -> WebGLTexture {
        self.inner.get("depthStencilTexture").as_::<WebGLTexture>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `motionVectorTexture` attribute.
    /// [`XRWebGLSubImage.motionVectorTexture`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/motionVectorTexture)
    pub fn motion_vector_texture(&self) -> WebGLTexture {
        self.inner.get("motionVectorTexture").as_::<WebGLTexture>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `imageIndex` attribute.
    /// [`XRWebGLSubImage.imageIndex`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/imageIndex)
    pub fn image_index(&self) -> u32 {
        self.inner.get("imageIndex").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `colorTextureWidth` attribute.
    /// [`XRWebGLSubImage.colorTextureWidth`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/colorTextureWidth)
    pub fn color_texture_width(&self) -> u32 {
        self.inner.get("colorTextureWidth").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `colorTextureHeight` attribute.
    /// [`XRWebGLSubImage.colorTextureHeight`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/colorTextureHeight)
    pub fn color_texture_height(&self) -> u32 {
        self.inner.get("colorTextureHeight").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `depthStencilTextureWidth` attribute.
    /// [`XRWebGLSubImage.depthStencilTextureWidth`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/depthStencilTextureWidth)
    pub fn depth_stencil_texture_width(&self) -> u32 {
        self.inner.get("depthStencilTextureWidth").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `depthStencilTextureHeight` attribute.
    /// [`XRWebGLSubImage.depthStencilTextureHeight`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/depthStencilTextureHeight)
    pub fn depth_stencil_texture_height(&self) -> u32 {
        self.inner.get("depthStencilTextureHeight").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `motionVectorTextureWidth` attribute.
    /// [`XRWebGLSubImage.motionVectorTextureWidth`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/motionVectorTextureWidth)
    pub fn motion_vector_texture_width(&self) -> u32 {
        self.inner.get("motionVectorTextureWidth").as_::<u32>()
    }
}
impl XRWebGLSubImage {
    /// Getter of the `motionVectorTextureHeight` attribute.
    /// [`XRWebGLSubImage.motionVectorTextureHeight`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLSubImage/motionVectorTextureHeight)
    pub fn motion_vector_texture_height(&self) -> u32 {
        self.inner.get("motionVectorTextureHeight").as_::<u32>()
    }
}
