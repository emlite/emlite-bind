use super::*;

/// The ChapterInformation class.
/// [`ChapterInformation`](https://developer.mozilla.org/en-US/docs/Web/API/ChapterInformation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChapterInformation {
    inner: Any,
}

impl FromVal for ChapterInformation {
    fn from_val(v: &Any) -> Self {
        ChapterInformation {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ChapterInformation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ChapterInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ChapterInformation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ChapterInformation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ChapterInformation> for Any {
    fn from(s: ChapterInformation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ChapterInformation> for Any {
    fn from(s: &ChapterInformation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ChapterInformation);

impl ChapterInformation {
    /// Getter of the `title` attribute.
    /// [`ChapterInformation.title`](https://developer.mozilla.org/en-US/docs/Web/API/ChapterInformation/title)
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }
}
impl ChapterInformation {
    /// Getter of the `startTime` attribute.
    /// [`ChapterInformation.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/ChapterInformation/startTime)
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }
}
impl ChapterInformation {
    /// Getter of the `artwork` attribute.
    /// [`ChapterInformation.artwork`](https://developer.mozilla.org/en-US/docs/Web/API/ChapterInformation/artwork)
    pub fn artwork(&self) -> TypedArray<MediaImage> {
        self.inner.get("artwork").as_::<TypedArray<MediaImage>>()
    }
}
