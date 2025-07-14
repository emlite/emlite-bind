use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImageBitmapRenderingContext {
    inner: emlite::Val,
}
impl FromVal for ImageBitmapRenderingContext {
    fn from_val(v: &emlite::Val) -> Self {
        ImageBitmapRenderingContext {
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
impl core::ops::Deref for ImageBitmapRenderingContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageBitmapRenderingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageBitmapRenderingContext> for emlite::Val {
    fn from(s: ImageBitmapRenderingContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageBitmapRenderingContext {
    pub fn canvas(&self) -> jsbind::Any {
        self.inner.get("canvas").as_::<jsbind::Any>()
    }
}
impl ImageBitmapRenderingContext {
    pub fn transfer_from_image_bitmap(&self, bitmap: ImageBitmap) -> jsbind::Undefined {
        self.inner
            .call("transferFromImageBitmap", &[bitmap.into()])
            .as_::<jsbind::Undefined>()
    }
}
