use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaImage {
    inner: Any,
}
impl FromVal for MediaImage {
    fn from_val(v: &Any) -> Self {
        MediaImage { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaImage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaImage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaImage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaImage> for Any {
    fn from(s: MediaImage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaImage> for Any {
    fn from(s: &MediaImage) -> Any {
        s.inner.clone()
    }
}

impl MediaImage {
    pub fn src(&self) -> String {
        self.inner.get("src").as_::<String>()
    }

    pub fn set_src(&mut self, value: &str) {
        self.inner.set("src", value);
    }
}
impl MediaImage {
    pub fn sizes(&self) -> String {
        self.inner.get("sizes").as_::<String>()
    }

    pub fn set_sizes(&mut self, value: &str) {
        self.inner.set("sizes", value);
    }
}
impl MediaImage {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
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
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
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
    pub fn artwork(&self) -> FrozenArray<MediaImage> {
        self.inner.get("artwork").as_::<FrozenArray<MediaImage>>()
    }
}
