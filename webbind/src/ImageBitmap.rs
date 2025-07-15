use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ImageBitmap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageBitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageBitmap {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageBitmap {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageBitmap> for emlite::Val {
    fn from(s: ImageBitmap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ImageBitmap);

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
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
