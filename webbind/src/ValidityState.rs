use super::*;

/// The ValidityState class.
/// [`ValidityState`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ValidityState {
    inner: Any,
}

impl FromVal for ValidityState {
    fn from_val(v: &Any) -> Self {
        ValidityState {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ValidityState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ValidityState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ValidityState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ValidityState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ValidityState> for Any {
    fn from(s: ValidityState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ValidityState> for Any {
    fn from(s: &ValidityState) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ValidityState);

impl ValidityState {
    /// Getter of the `valueMissing` attribute.
    /// [`ValidityState.valueMissing`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing)
    pub fn value_missing(&self) -> bool {
        self.inner.get("valueMissing").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `typeMismatch` attribute.
    /// [`ValidityState.typeMismatch`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch)
    pub fn type_mismatch(&self) -> bool {
        self.inner.get("typeMismatch").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `patternMismatch` attribute.
    /// [`ValidityState.patternMismatch`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch)
    pub fn pattern_mismatch(&self) -> bool {
        self.inner.get("patternMismatch").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `tooLong` attribute.
    /// [`ValidityState.tooLong`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong)
    pub fn too_long(&self) -> bool {
        self.inner.get("tooLong").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `tooShort` attribute.
    /// [`ValidityState.tooShort`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort)
    pub fn too_short(&self) -> bool {
        self.inner.get("tooShort").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `rangeUnderflow` attribute.
    /// [`ValidityState.rangeUnderflow`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow)
    pub fn range_underflow(&self) -> bool {
        self.inner.get("rangeUnderflow").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `rangeOverflow` attribute.
    /// [`ValidityState.rangeOverflow`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow)
    pub fn range_overflow(&self) -> bool {
        self.inner.get("rangeOverflow").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `stepMismatch` attribute.
    /// [`ValidityState.stepMismatch`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch)
    pub fn step_mismatch(&self) -> bool {
        self.inner.get("stepMismatch").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `badInput` attribute.
    /// [`ValidityState.badInput`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput)
    pub fn bad_input(&self) -> bool {
        self.inner.get("badInput").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `customError` attribute.
    /// [`ValidityState.customError`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError)
    pub fn custom_error(&self) -> bool {
        self.inner.get("customError").as_::<bool>()
    }
}
impl ValidityState {
    /// Getter of the `valid` attribute.
    /// [`ValidityState.valid`](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid)
    pub fn valid(&self) -> bool {
        self.inner.get("valid").as_::<bool>()
    }
}
