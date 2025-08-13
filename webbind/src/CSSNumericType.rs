use super::*;




/// The CSSNumericType dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSNumericType {
    inner: Any,
}

impl FromVal for CSSNumericType {
    fn from_val(v: &Any) -> Self {
        CSSNumericType { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSNumericType {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSNumericType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSNumericType {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSNumericType {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSNumericType> for Any {
    fn from(s: CSSNumericType) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSNumericType> for Any {
    fn from(s: &CSSNumericType) -> Any {
        s.inner.clone()
    }
}

impl CSSNumericType {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> i32 {
        self.inner.get("length").as_::<i32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: i32) {
        self.inner.set("length", value);
    }
}
impl CSSNumericType {
    /// Getter of the `angle` attribute.
    pub fn angle(&self) -> i32 {
        self.inner.get("angle").as_::<i32>()
    }

    /// Setter of the `angle` attribute.
    pub fn set_angle(&mut self, value: i32) {
        self.inner.set("angle", value);
    }
}
impl CSSNumericType {
    /// Getter of the `time` attribute.
    pub fn time(&self) -> i32 {
        self.inner.get("time").as_::<i32>()
    }

    /// Setter of the `time` attribute.
    pub fn set_time(&mut self, value: i32) {
        self.inner.set("time", value);
    }
}
impl CSSNumericType {
    /// Getter of the `frequency` attribute.
    pub fn frequency(&self) -> i32 {
        self.inner.get("frequency").as_::<i32>()
    }

    /// Setter of the `frequency` attribute.
    pub fn set_frequency(&mut self, value: i32) {
        self.inner.set("frequency", value);
    }
}
impl CSSNumericType {
    /// Getter of the `resolution` attribute.
    pub fn resolution(&self) -> i32 {
        self.inner.get("resolution").as_::<i32>()
    }

    /// Setter of the `resolution` attribute.
    pub fn set_resolution(&mut self, value: i32) {
        self.inner.set("resolution", value);
    }
}
impl CSSNumericType {
    /// Getter of the `flex` attribute.
    pub fn flex(&self) -> i32 {
        self.inner.get("flex").as_::<i32>()
    }

    /// Setter of the `flex` attribute.
    pub fn set_flex(&mut self, value: i32) {
        self.inner.set("flex", value);
    }
}
impl CSSNumericType {
    /// Getter of the `percent` attribute.
    pub fn percent(&self) -> i32 {
        self.inner.get("percent").as_::<i32>()
    }

    /// Setter of the `percent` attribute.
    pub fn set_percent(&mut self, value: i32) {
        self.inner.set("percent", value);
    }
}
impl CSSNumericType {
    /// Getter of the `percentHint` attribute.
    pub fn percent_hint(&self) -> CSSNumericBaseType {
        self.inner.get("percentHint").as_::<CSSNumericBaseType>()
    }

    /// Setter of the `percentHint` attribute.
    pub fn set_percent_hint(&mut self, value: &CSSNumericBaseType) {
        self.inner.set("percentHint", value);
    }
}
