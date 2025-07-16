use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoColorSpaceInit {
    inner: Any,
}
impl FromVal for VideoColorSpaceInit {
    fn from_val(v: &Any) -> Self {
        VideoColorSpaceInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoColorSpaceInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoColorSpaceInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoColorSpaceInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoColorSpaceInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoColorSpaceInit> for Any {
    fn from(s: VideoColorSpaceInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoColorSpaceInit> for Any {
    fn from(s: &VideoColorSpaceInit) -> Any {
        s.inner.clone()
    }
}

impl VideoColorSpaceInit {
    pub fn primaries(&self) -> VideoColorPrimaries {
        self.inner.get("primaries").as_::<VideoColorPrimaries>()
    }

    pub fn set_primaries(&mut self, value: &VideoColorPrimaries) {
        self.inner.set("primaries", value);
    }
}
impl VideoColorSpaceInit {
    pub fn transfer(&self) -> VideoTransferCharacteristics {
        self.inner
            .get("transfer")
            .as_::<VideoTransferCharacteristics>()
    }

    pub fn set_transfer(&mut self, value: &VideoTransferCharacteristics) {
        self.inner.set("transfer", value);
    }
}
impl VideoColorSpaceInit {
    pub fn matrix(&self) -> VideoMatrixCoefficients {
        self.inner.get("matrix").as_::<VideoMatrixCoefficients>()
    }

    pub fn set_matrix(&mut self, value: &VideoMatrixCoefficients) {
        self.inner.set("matrix", value);
    }
}
impl VideoColorSpaceInit {
    pub fn full_range(&self) -> bool {
        self.inner.get("fullRange").as_::<bool>()
    }

    pub fn set_full_range(&mut self, value: bool) {
        self.inner.set("fullRange", value);
    }
}
/// The VideoColorSpace class.
/// [`VideoColorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoColorSpace {
    inner: Any,
}
impl FromVal for VideoColorSpace {
    fn from_val(v: &Any) -> Self {
        VideoColorSpace {
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
impl core::ops::Deref for VideoColorSpace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoColorSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoColorSpace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoColorSpace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoColorSpace> for Any {
    fn from(s: VideoColorSpace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoColorSpace> for Any {
    fn from(s: &VideoColorSpace) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoColorSpace);

impl VideoColorSpace {
    /// The `new VideoColorSpace(..)` constructor, creating a new VideoColorSpace instance
    pub fn new0() -> VideoColorSpace {
        Self {
            inner: Any::global("VideoColorSpace").new(&[]).as_::<Any>(),
        }
    }

    /// The `new VideoColorSpace(..)` constructor, creating a new VideoColorSpace instance
    pub fn new1(init: &VideoColorSpaceInit) -> VideoColorSpace {
        Self {
            inner: Any::global("VideoColorSpace")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl VideoColorSpace {
    /// Getter of the `primaries` attribute.
    /// [`VideoColorSpace.primaries`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/primaries)
    pub fn primaries(&self) -> VideoColorPrimaries {
        self.inner.get("primaries").as_::<VideoColorPrimaries>()
    }
}
impl VideoColorSpace {
    /// Getter of the `transfer` attribute.
    /// [`VideoColorSpace.transfer`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/transfer)
    pub fn transfer(&self) -> VideoTransferCharacteristics {
        self.inner
            .get("transfer")
            .as_::<VideoTransferCharacteristics>()
    }
}
impl VideoColorSpace {
    /// Getter of the `matrix` attribute.
    /// [`VideoColorSpace.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/matrix)
    pub fn matrix(&self) -> VideoMatrixCoefficients {
        self.inner.get("matrix").as_::<VideoMatrixCoefficients>()
    }
}
impl VideoColorSpace {
    /// Getter of the `fullRange` attribute.
    /// [`VideoColorSpace.fullRange`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/fullRange)
    pub fn full_range(&self) -> bool {
        self.inner.get("fullRange").as_::<bool>()
    }
}
impl VideoColorSpace {
    /// The toJSON method.
    /// [`VideoColorSpace.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/toJSON)
    pub fn to_json(&self) -> VideoColorSpaceInit {
        self.inner.call("toJSON", &[]).as_::<VideoColorSpaceInit>()
    }
}
