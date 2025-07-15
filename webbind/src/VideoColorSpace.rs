use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoColorSpaceInit {
    inner: emlite::Val,
}
impl FromVal for VideoColorSpaceInit {
    fn from_val(v: &emlite::Val) -> Self {
        VideoColorSpaceInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoColorSpaceInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoColorSpaceInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoColorSpaceInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoColorSpaceInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoColorSpaceInit> for emlite::Val {
    fn from(s: VideoColorSpaceInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&VideoColorSpaceInit> for emlite::Val {
    fn from(s: &VideoColorSpaceInit) -> emlite::Val {
        s.inner.clone()
    }
}

impl VideoColorSpaceInit {
    pub fn primaries(&self) -> VideoColorPrimaries {
        self.inner.get("primaries").as_::<VideoColorPrimaries>()
    }

    pub fn set_primaries(&mut self, value: VideoColorPrimaries) {
        self.inner.set("primaries", value);
    }
}
impl VideoColorSpaceInit {
    pub fn transfer(&self) -> VideoTransferCharacteristics {
        self.inner
            .get("transfer")
            .as_::<VideoTransferCharacteristics>()
    }

    pub fn set_transfer(&mut self, value: VideoTransferCharacteristics) {
        self.inner.set("transfer", value);
    }
}
impl VideoColorSpaceInit {
    pub fn matrix(&self) -> VideoMatrixCoefficients {
        self.inner.get("matrix").as_::<VideoMatrixCoefficients>()
    }

    pub fn set_matrix(&mut self, value: VideoMatrixCoefficients) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoColorSpace {
    inner: emlite::Val,
}
impl FromVal for VideoColorSpace {
    fn from_val(v: &emlite::Val) -> Self {
        VideoColorSpace {
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
impl core::ops::Deref for VideoColorSpace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoColorSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoColorSpace {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoColorSpace {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoColorSpace> for emlite::Val {
    fn from(s: VideoColorSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&VideoColorSpace> for emlite::Val {
    fn from(s: &VideoColorSpace) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoColorSpace);

impl VideoColorSpace {
    pub fn new0() -> VideoColorSpace {
        Self {
            inner: emlite::Val::global("VideoColorSpace")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: VideoColorSpaceInit) -> VideoColorSpace {
        Self {
            inner: emlite::Val::global("VideoColorSpace")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl VideoColorSpace {
    pub fn primaries(&self) -> VideoColorPrimaries {
        self.inner.get("primaries").as_::<VideoColorPrimaries>()
    }
}
impl VideoColorSpace {
    pub fn transfer(&self) -> VideoTransferCharacteristics {
        self.inner
            .get("transfer")
            .as_::<VideoTransferCharacteristics>()
    }
}
impl VideoColorSpace {
    pub fn matrix(&self) -> VideoMatrixCoefficients {
        self.inner.get("matrix").as_::<VideoMatrixCoefficients>()
    }
}
impl VideoColorSpace {
    pub fn full_range(&self) -> bool {
        self.inner.get("fullRange").as_::<bool>()
    }
}
impl VideoColorSpace {
    pub fn to_json(&self) -> VideoColorSpaceInit {
        self.inner.call("toJSON", &[]).as_::<VideoColorSpaceInit>()
    }
}
