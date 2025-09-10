use super::*;

/// The MediaKeyStatusMap class.
/// [`MediaKeyStatusMap`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeyStatusMap {
    inner: Any,
}

impl FromVal for MediaKeyStatusMap {
    fn from_val(v: &Any) -> Self {
        MediaKeyStatusMap {
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

impl core::ops::Deref for MediaKeyStatusMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaKeyStatusMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaKeyStatusMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaKeyStatusMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaKeyStatusMap> for Any {
    fn from(s: MediaKeyStatusMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaKeyStatusMap> for Any {
    fn from(s: &MediaKeyStatusMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaKeyStatusMap);

impl MediaKeyStatusMap {
    /// Getter of the `size` attribute.
    /// [`MediaKeyStatusMap.size`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl MediaKeyStatusMap {
    /// The has method.
    /// [`MediaKeyStatusMap.has`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)
    pub fn has(&self, key_id: &Any) -> bool {
        self.inner.call("has", &[key_id.into()]).as_::<bool>()
    }
}
impl MediaKeyStatusMap {
    /// The get method.
    /// [`MediaKeyStatusMap.get`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)
    pub fn get(&self, key_id: &Any) -> Any {
        self.inner.call("get", &[key_id.into()]).as_::<Any>()
    }
}
