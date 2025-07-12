use super::*;

#[derive(Clone, Debug)]
pub struct ImageBitmap {
    inner: emlite::Val,
}
impl FromVal for ImageBitmap {
    fn from_val(v: &emlite::Val) -> Self {
        ImageBitmap {
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
impl std::ops::Deref for ImageBitmap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageBitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageBitmap> for emlite::Val {
    fn from(s: ImageBitmap) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageBitmap {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl ImageBitmap {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
impl ImageBitmap {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
