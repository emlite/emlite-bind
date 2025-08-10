use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLRankRange {
    inner: Any,
}
impl FromVal for MLRankRange {
    fn from_val(v: &Any) -> Self {
        MLRankRange { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLRankRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLRankRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLRankRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLRankRange {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLRankRange> for Any {
    fn from(s: MLRankRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLRankRange> for Any {
    fn from(s: &MLRankRange) -> Any {
        s.inner.clone()
    }
}

impl MLRankRange {
    pub fn min(&self) -> u32 {
        self.inner.get("min").as_::<u32>()
    }

    pub fn set_min(&mut self, value: u32) {
        self.inner.set("min", value);
    }
}
impl MLRankRange {
    pub fn max(&self) -> u32 {
        self.inner.get("max").as_::<u32>()
    }

    pub fn set_max(&mut self, value: u32) {
        self.inner.set("max", value);
    }
}
