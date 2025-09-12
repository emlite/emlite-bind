use super::*;

/// The XRReferenceSpace class.
/// [`XRReferenceSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRReferenceSpace {
    inner: XRSpace,
}

impl FromVal for XRReferenceSpace {
    fn from_val(v: &Any) -> Self {
        XRReferenceSpace {
            inner: XRSpace::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRReferenceSpace {
    type Target = XRSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRReferenceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRReferenceSpace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRReferenceSpace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRReferenceSpace> for Any {
    fn from(s: XRReferenceSpace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRReferenceSpace> for Any {
    fn from(s: &XRReferenceSpace) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRReferenceSpace);

impl XRReferenceSpace {
    /// Getter of the `onreset` attribute.
    /// [`XRReferenceSpace.onreset`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpace/onreset)
    pub fn onreset(&self) -> Any {
        self.inner.get("onreset").as_::<Any>()
    }

    /// Setter of the `onreset` attribute.
    /// [`XRReferenceSpace.onreset`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpace/onreset)
    pub fn set_onreset(&mut self, value: &Any) {
        self.inner.set("onreset", value);
    }
}
impl XRReferenceSpace {
    /// The getOffsetReferenceSpace method.
    /// [`XRReferenceSpace.getOffsetReferenceSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpace/getOffsetReferenceSpace)
    pub fn get_offset_reference_space(&self, origin_offset: &XRRigidTransform) -> XRReferenceSpace {
        self.inner
            .call("getOffsetReferenceSpace", &[origin_offset.into()])
            .as_::<XRReferenceSpace>()
    }
}
