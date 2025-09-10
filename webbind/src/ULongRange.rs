use super::*;

/// The ULongRange dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ULongRange {
    inner: Any,
}

impl FromVal for ULongRange {
    fn from_val(v: &Any) -> Self {
        ULongRange { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ULongRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ULongRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ULongRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ULongRange {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ULongRange> for Any {
    fn from(s: ULongRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ULongRange> for Any {
    fn from(s: &ULongRange) -> Any {
        s.inner.clone()
    }
}

impl ULongRange {
    /// Getter of the `max` attribute.
    pub fn max(&self) -> u32 {
        self.inner.get("max").as_::<u32>()
    }

    /// Setter of the `max` attribute.
    pub fn set_max(&mut self, value: u32) {
        self.inner.set("max", value);
    }
}
impl ULongRange {
    /// Getter of the `min` attribute.
    pub fn min(&self) -> u32 {
        self.inner.get("min").as_::<u32>()
    }

    /// Setter of the `min` attribute.
    pub fn set_min(&mut self, value: u32) {
        self.inner.set("min", value);
    }
}
