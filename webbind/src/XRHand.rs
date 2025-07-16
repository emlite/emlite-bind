use super::*;

/// The XRHand class.
/// [`XRHand`](https://developer.mozilla.org/en-US/docs/Web/API/XRHand)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHand {
    inner: Any,
}
impl FromVal for XRHand {
    fn from_val(v: &Any) -> Self {
        XRHand {
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
impl core::ops::Deref for XRHand {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRHand {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRHand {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRHand> for Any {
    fn from(s: XRHand) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRHand> for Any {
    fn from(s: &XRHand) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRHand);

impl XRHand {
    /// Getter of the `size` attribute.
    /// [`XRHand.size`](https://developer.mozilla.org/en-US/docs/Web/API/XRHand/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl XRHand {
    /// The get method.
    /// [`XRHand.get`](https://developer.mozilla.org/en-US/docs/Web/API/XRHand/get)
    pub fn get(&self, key: &XRHandJoint) -> XRJointSpace {
        self.inner.call("get", &[key.into()]).as_::<XRJointSpace>()
    }
}
