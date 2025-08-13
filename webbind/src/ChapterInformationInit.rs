use super::*;




/// The ChapterInformationInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChapterInformationInit {
    inner: Any,
}

impl FromVal for ChapterInformationInit {
    fn from_val(v: &Any) -> Self {
        ChapterInformationInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ChapterInformationInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ChapterInformationInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ChapterInformationInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ChapterInformationInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ChapterInformationInit> for Any {
    fn from(s: ChapterInformationInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ChapterInformationInit> for Any {
    fn from(s: &ChapterInformationInit) -> Any {
        s.inner.clone()
    }
}

impl ChapterInformationInit {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl ChapterInformationInit {
    /// Getter of the `startTime` attribute.
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }

    /// Setter of the `startTime` attribute.
    pub fn set_start_time(&mut self, value: f64) {
        self.inner.set("startTime", value);
    }
}
impl ChapterInformationInit {
    /// Getter of the `artwork` attribute.
    pub fn artwork(&self) -> TypedArray<MediaImage> {
        self.inner.get("artwork").as_::<TypedArray<MediaImage>>()
    }

    /// Setter of the `artwork` attribute.
    pub fn set_artwork(&mut self, value: &TypedArray<MediaImage>) {
        self.inner.set("artwork", value);
    }
}
