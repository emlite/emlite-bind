use super::*;

/// The MLSplitOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLSplitOptions {
    inner: Any,
}

impl FromVal for MLSplitOptions {
    fn from_val(v: &Any) -> Self {
        MLSplitOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLSplitOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLSplitOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLSplitOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLSplitOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLSplitOptions> for Any {
    fn from(s: MLSplitOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLSplitOptions> for Any {
    fn from(s: &MLSplitOptions) -> Any {
        s.inner.clone()
    }
}

impl MLSplitOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
