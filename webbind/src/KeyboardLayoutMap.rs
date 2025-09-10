use super::*;

/// The KeyboardLayoutMap class.
/// [`KeyboardLayoutMap`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardLayoutMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyboardLayoutMap {
    inner: Any,
}

impl FromVal for KeyboardLayoutMap {
    fn from_val(v: &Any) -> Self {
        KeyboardLayoutMap {
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

impl core::ops::Deref for KeyboardLayoutMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeyboardLayoutMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeyboardLayoutMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeyboardLayoutMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KeyboardLayoutMap> for Any {
    fn from(s: KeyboardLayoutMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeyboardLayoutMap> for Any {
    fn from(s: &KeyboardLayoutMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(KeyboardLayoutMap);
