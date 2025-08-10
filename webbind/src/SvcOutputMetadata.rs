use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SvcOutputMetadata {
    inner: Any,
}
impl FromVal for SvcOutputMetadata {
    fn from_val(v: &Any) -> Self {
        SvcOutputMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SvcOutputMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SvcOutputMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SvcOutputMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SvcOutputMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SvcOutputMetadata> for Any {
    fn from(s: SvcOutputMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SvcOutputMetadata> for Any {
    fn from(s: &SvcOutputMetadata) -> Any {
        s.inner.clone()
    }
}

impl SvcOutputMetadata {
    pub fn temporal_layer_id(&self) -> u32 {
        self.inner.get("temporalLayerId").as_::<u32>()
    }

    pub fn set_temporal_layer_id(&mut self, value: u32) {
        self.inner.set("temporalLayerId", value);
    }
}
