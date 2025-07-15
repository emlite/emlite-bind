use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLSubImage {
    inner: XRSubImage,
}
impl FromVal for XRWebGLSubImage {
    fn from_val(v: &emlite::Val) -> Self {
        XRWebGLSubImage { inner: XRSubImage::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for XRWebGLSubImage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRWebGLSubImage {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRWebGLSubImage> for emlite::Val {
    fn from(s: XRWebGLSubImage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRWebGLSubImage);


impl XRWebGLSubImage {
    pub fn color_texture(&self) -> WebGLTexture {
        self.inner.get("colorTexture").as_::<WebGLTexture>()
    }

}
impl XRWebGLSubImage {
    pub fn depth_stencil_texture(&self) -> WebGLTexture {
        self.inner.get("depthStencilTexture").as_::<WebGLTexture>()
    }

}
impl XRWebGLSubImage {
    pub fn motion_vector_texture(&self) -> WebGLTexture {
        self.inner.get("motionVectorTexture").as_::<WebGLTexture>()
    }

}
impl XRWebGLSubImage {
    pub fn image_index(&self) -> u32 {
        self.inner.get("imageIndex").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn color_texture_width(&self) -> u32 {
        self.inner.get("colorTextureWidth").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn color_texture_height(&self) -> u32 {
        self.inner.get("colorTextureHeight").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn depth_stencil_texture_width(&self) -> u32 {
        self.inner.get("depthStencilTextureWidth").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn depth_stencil_texture_height(&self) -> u32 {
        self.inner.get("depthStencilTextureHeight").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn motion_vector_texture_width(&self) -> u32 {
        self.inner.get("motionVectorTextureWidth").as_::<u32>()
    }

}
impl XRWebGLSubImage {
    pub fn motion_vector_texture_height(&self) -> u32 {
        self.inner.get("motionVectorTextureHeight").as_::<u32>()
    }

}
