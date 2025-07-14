use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KeyboardLayoutMap {
    inner: emlite::Val,
}
impl FromVal for KeyboardLayoutMap {
    fn from_val(v: &emlite::Val) -> Self {
        KeyboardLayoutMap {
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
impl core::ops::Deref for KeyboardLayoutMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for KeyboardLayoutMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KeyboardLayoutMap> for emlite::Val {
    fn from(s: KeyboardLayoutMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
