use super::*;




/// The ValidityStateFlags dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ValidityStateFlags {
    inner: Any,
}

impl FromVal for ValidityStateFlags {
    fn from_val(v: &Any) -> Self {
        ValidityStateFlags { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ValidityStateFlags {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ValidityStateFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ValidityStateFlags {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ValidityStateFlags {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ValidityStateFlags> for Any {
    fn from(s: ValidityStateFlags) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ValidityStateFlags> for Any {
    fn from(s: &ValidityStateFlags) -> Any {
        s.inner.clone()
    }
}

impl ValidityStateFlags {
    /// Getter of the `valueMissing` attribute.
    pub fn value_missing(&self) -> bool {
        self.inner.get("valueMissing").as_::<bool>()
    }

    /// Setter of the `valueMissing` attribute.
    pub fn set_value_missing(&mut self, value: bool) {
        self.inner.set("valueMissing", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `typeMismatch` attribute.
    pub fn type_mismatch(&self) -> bool {
        self.inner.get("typeMismatch").as_::<bool>()
    }

    /// Setter of the `typeMismatch` attribute.
    pub fn set_type_mismatch(&mut self, value: bool) {
        self.inner.set("typeMismatch", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `patternMismatch` attribute.
    pub fn pattern_mismatch(&self) -> bool {
        self.inner.get("patternMismatch").as_::<bool>()
    }

    /// Setter of the `patternMismatch` attribute.
    pub fn set_pattern_mismatch(&mut self, value: bool) {
        self.inner.set("patternMismatch", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `tooLong` attribute.
    pub fn too_long(&self) -> bool {
        self.inner.get("tooLong").as_::<bool>()
    }

    /// Setter of the `tooLong` attribute.
    pub fn set_too_long(&mut self, value: bool) {
        self.inner.set("tooLong", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `tooShort` attribute.
    pub fn too_short(&self) -> bool {
        self.inner.get("tooShort").as_::<bool>()
    }

    /// Setter of the `tooShort` attribute.
    pub fn set_too_short(&mut self, value: bool) {
        self.inner.set("tooShort", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `rangeUnderflow` attribute.
    pub fn range_underflow(&self) -> bool {
        self.inner.get("rangeUnderflow").as_::<bool>()
    }

    /// Setter of the `rangeUnderflow` attribute.
    pub fn set_range_underflow(&mut self, value: bool) {
        self.inner.set("rangeUnderflow", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `rangeOverflow` attribute.
    pub fn range_overflow(&self) -> bool {
        self.inner.get("rangeOverflow").as_::<bool>()
    }

    /// Setter of the `rangeOverflow` attribute.
    pub fn set_range_overflow(&mut self, value: bool) {
        self.inner.set("rangeOverflow", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `stepMismatch` attribute.
    pub fn step_mismatch(&self) -> bool {
        self.inner.get("stepMismatch").as_::<bool>()
    }

    /// Setter of the `stepMismatch` attribute.
    pub fn set_step_mismatch(&mut self, value: bool) {
        self.inner.set("stepMismatch", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `badInput` attribute.
    pub fn bad_input(&self) -> bool {
        self.inner.get("badInput").as_::<bool>()
    }

    /// Setter of the `badInput` attribute.
    pub fn set_bad_input(&mut self, value: bool) {
        self.inner.set("badInput", value);
    }
}
impl ValidityStateFlags {
    /// Getter of the `customError` attribute.
    pub fn custom_error(&self) -> bool {
        self.inner.get("customError").as_::<bool>()
    }

    /// Setter of the `customError` attribute.
    pub fn set_custom_error(&mut self, value: bool) {
        self.inner.set("customError", value);
    }
}
