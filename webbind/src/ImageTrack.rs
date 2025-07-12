use super::*;

#[derive(Clone, Debug)]
pub struct ImageTrack {
    inner: emlite::Val,
}
impl FromVal for ImageTrack {
    fn from_val(v: &emlite::Val) -> Self {
        ImageTrack {
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
impl std::ops::Deref for ImageTrack {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageTrack> for emlite::Val {
    fn from(s: ImageTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageTrack {
    pub fn animated(&self) -> bool {
        self.inner.get("animated").as_::<bool>()
    }
}
impl ImageTrack {
    pub fn frame_count(&self) -> u32 {
        self.inner.get("frameCount").as_::<u32>()
    }
}
impl ImageTrack {
    pub fn repetition_count(&self) -> f32 {
        self.inner.get("repetitionCount").as_::<f32>()
    }
}
impl ImageTrack {
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
