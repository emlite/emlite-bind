use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ValidityState {
    inner: emlite::Val,
}
impl FromVal for ValidityState {
    fn from_val(v: &emlite::Val) -> Self {
        ValidityState {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ValidityState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ValidityState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ValidityState {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ValidityState {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ValidityState> for emlite::Val {
    fn from(s: ValidityState) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ValidityState> for emlite::Val {
    fn from(s: &ValidityState) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ValidityState);

impl ValidityState {
    pub fn value_missing(&self) -> bool {
        self.inner.get("valueMissing").as_::<bool>()
    }
}
impl ValidityState {
    pub fn type_mismatch(&self) -> bool {
        self.inner.get("typeMismatch").as_::<bool>()
    }
}
impl ValidityState {
    pub fn pattern_mismatch(&self) -> bool {
        self.inner.get("patternMismatch").as_::<bool>()
    }
}
impl ValidityState {
    pub fn too_long(&self) -> bool {
        self.inner.get("tooLong").as_::<bool>()
    }
}
impl ValidityState {
    pub fn too_short(&self) -> bool {
        self.inner.get("tooShort").as_::<bool>()
    }
}
impl ValidityState {
    pub fn range_underflow(&self) -> bool {
        self.inner.get("rangeUnderflow").as_::<bool>()
    }
}
impl ValidityState {
    pub fn range_overflow(&self) -> bool {
        self.inner.get("rangeOverflow").as_::<bool>()
    }
}
impl ValidityState {
    pub fn step_mismatch(&self) -> bool {
        self.inner.get("stepMismatch").as_::<bool>()
    }
}
impl ValidityState {
    pub fn bad_input(&self) -> bool {
        self.inner.get("badInput").as_::<bool>()
    }
}
impl ValidityState {
    pub fn custom_error(&self) -> bool {
        self.inner.get("customError").as_::<bool>()
    }
}
impl ValidityState {
    pub fn valid(&self) -> bool {
        self.inner.get("valid").as_::<bool>()
    }
}
