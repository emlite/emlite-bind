use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CustomStateSet {
    inner: emlite::Val,
}
impl FromVal for CustomStateSet {
    fn from_val(v: &emlite::Val) -> Self {
        CustomStateSet {
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
impl core::ops::Deref for CustomStateSet {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CustomStateSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CustomStateSet> for emlite::Val {
    fn from(s: CustomStateSet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
