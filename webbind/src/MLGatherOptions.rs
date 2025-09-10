use super::*;

/// The MLGatherOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGatherOptions {
    inner: Any,
}

impl FromVal for MLGatherOptions {
    fn from_val(v: &Any) -> Self {
        MLGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGatherOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGatherOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGatherOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLGatherOptions> for Any {
    fn from(s: MLGatherOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGatherOptions> for Any {
    fn from(s: &MLGatherOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGatherOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
