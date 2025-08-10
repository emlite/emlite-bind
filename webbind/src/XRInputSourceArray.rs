use super::*;

/// The XRInputSourceArray class.
/// [`XRInputSourceArray`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceArray)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourceArray {
    inner: Any,
}

impl FromVal for XRInputSourceArray {
    fn from_val(v: &Any) -> Self {
        XRInputSourceArray {
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

impl core::ops::Deref for XRInputSourceArray {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRInputSourceArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRInputSourceArray {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRInputSourceArray {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRInputSourceArray> for Any {
    fn from(s: XRInputSourceArray) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRInputSourceArray> for Any {
    fn from(s: &XRInputSourceArray) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRInputSourceArray);

impl XRInputSourceArray {
    /// Getter of the `length` attribute.
    /// [`XRInputSourceArray.length`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceArray/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
