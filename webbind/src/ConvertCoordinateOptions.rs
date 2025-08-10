use super::*;

/// The ConvertCoordinateOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConvertCoordinateOptions {
    inner: Any,
}

impl FromVal for ConvertCoordinateOptions {
    fn from_val(v: &Any) -> Self {
        ConvertCoordinateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ConvertCoordinateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ConvertCoordinateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ConvertCoordinateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ConvertCoordinateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ConvertCoordinateOptions> for Any {
    fn from(s: ConvertCoordinateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ConvertCoordinateOptions> for Any {
    fn from(s: &ConvertCoordinateOptions) -> Any {
        s.inner.clone()
    }
}

impl ConvertCoordinateOptions {
    /// Getter of the `fromBox` attribute.
    pub fn from_box(&self) -> CSSBoxType {
        self.inner.get("fromBox").as_::<CSSBoxType>()
    }

    /// Setter of the `fromBox` attribute.
    pub fn set_from_box(&mut self, value: &CSSBoxType) {
        self.inner.set("fromBox", value);
    }
}
impl ConvertCoordinateOptions {
    /// Getter of the `toBox` attribute.
    pub fn to_box(&self) -> CSSBoxType {
        self.inner.get("toBox").as_::<CSSBoxType>()
    }

    /// Setter of the `toBox` attribute.
    pub fn set_to_box(&mut self, value: &CSSBoxType) {
        self.inner.set("toBox", value);
    }
}
