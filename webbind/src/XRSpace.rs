use super::*;

/// The XRSpace class.
/// [`XRSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRSpace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSpace {
    inner: EventTarget,
}

impl FromVal for XRSpace {
    fn from_val(v: &Any) -> Self {
        XRSpace {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRSpace {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRSpace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRSpace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRSpace> for Any {
    fn from(s: XRSpace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRSpace> for Any {
    fn from(s: &XRSpace) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRSpace);
