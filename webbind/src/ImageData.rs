use super::*;

#[derive(Clone, Debug)]
pub struct ImageData {
    inner: emlite::Val,
}
impl FromVal for ImageData {
    fn from_val(v: &emlite::Val) -> Self {
        ImageData {
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
impl std::ops::Deref for ImageData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageData> for emlite::Val {
    fn from(s: ImageData) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageData {
    pub fn new0(data: jsbind::Any, sw: u32) -> ImageData {
        Self {
            inner: emlite::Val::global("ImageData")
                .new(&[data.into(), sw.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(data: jsbind::Any, sw: u32, sh: u32) -> ImageData {
        Self {
            inner: emlite::Val::global("ImageData")
                .new(&[data.into(), sw.into(), sh.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(data: jsbind::Any, sw: u32, sh: u32, settings: ImageDataSettings) -> ImageData {
        Self {
            inner: emlite::Val::global("ImageData")
                .new(&[data.into(), sw.into(), sh.into(), settings.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ImageData {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl ImageData {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
impl ImageData {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl ImageData {
    pub fn pixel_format(&self) -> ImageDataPixelFormat {
        self.inner.get("pixelFormat").as_::<ImageDataPixelFormat>()
    }
}
impl ImageData {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }
}
