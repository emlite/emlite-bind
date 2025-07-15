use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OverconstrainedError {
    inner: DOMException,
}
impl FromVal for OverconstrainedError {
    fn from_val(v: &emlite::Val) -> Self {
        OverconstrainedError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OverconstrainedError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OverconstrainedError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OverconstrainedError {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OverconstrainedError {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OverconstrainedError> for emlite::Val {
    fn from(s: OverconstrainedError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(OverconstrainedError);

impl OverconstrainedError {
    pub fn new0(constraint: DOMString) -> OverconstrainedError {
        Self {
            inner: emlite::Val::global("OverconstrainedError")
                .new(&[constraint.into()])
                .as_::<DOMException>(),
        }
    }

    pub fn new1(constraint: DOMString, message: DOMString) -> OverconstrainedError {
        Self {
            inner: emlite::Val::global("OverconstrainedError")
                .new(&[constraint.into(), message.into()])
                .as_::<DOMException>(),
        }
    }
}
impl OverconstrainedError {
    pub fn constraint(&self) -> DOMString {
        self.inner.get("constraint").as_::<DOMString>()
    }
}
