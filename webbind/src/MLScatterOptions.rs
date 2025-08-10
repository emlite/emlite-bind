use super::*;

/// The MLScatterOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLScatterOptions {
    inner: Any,
}

impl FromVal for MLScatterOptions {
    fn from_val(v: &Any) -> Self {
        MLScatterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLScatterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLScatterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLScatterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLScatterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLScatterOptions> for Any {
    fn from(s: MLScatterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLScatterOptions> for Any {
    fn from(s: &MLScatterOptions) -> Any {
        s.inner.clone()
    }
}

impl MLScatterOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
