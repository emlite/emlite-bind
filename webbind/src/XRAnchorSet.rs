use super::*;

/// The XRAnchorSet class.
/// [`XRAnchorSet`](https://developer.mozilla.org/en-US/docs/Web/API/XRAnchorSet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRAnchorSet {
    inner: Any,
}

impl FromVal for XRAnchorSet {
    fn from_val(v: &Any) -> Self {
        XRAnchorSet {
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

impl core::ops::Deref for XRAnchorSet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRAnchorSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRAnchorSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRAnchorSet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRAnchorSet> for Any {
    fn from(s: XRAnchorSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRAnchorSet> for Any {
    fn from(s: &XRAnchorSet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRAnchorSet);
