use super::*;

/// The XRAnchor class.
/// [`XRAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/XRAnchor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRAnchor {
    inner: Any,
}
impl FromVal for XRAnchor {
    fn from_val(v: &Any) -> Self {
        XRAnchor {
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
impl core::ops::Deref for XRAnchor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRAnchor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRAnchor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRAnchor> for Any {
    fn from(s: XRAnchor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRAnchor> for Any {
    fn from(s: &XRAnchor) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRAnchor);

impl XRAnchor {
    /// Getter of the `anchorSpace` attribute.
    /// [`XRAnchor.anchorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRAnchor/anchorSpace)
    pub fn anchor_space(&self) -> XRSpace {
        self.inner.get("anchorSpace").as_::<XRSpace>()
    }
}
impl XRAnchor {
    /// The requestPersistentHandle method.
    /// [`XRAnchor.requestPersistentHandle`](https://developer.mozilla.org/en-US/docs/Web/API/XRAnchor/requestPersistentHandle)
    pub fn request_persistent_handle(&self) -> Promise<JsString> {
        self.inner
            .call("requestPersistentHandle", &[])
            .as_::<Promise<JsString>>()
    }
}
impl XRAnchor {
    /// The delete method.
    /// [`XRAnchor.delete`](https://developer.mozilla.org/en-US/docs/Web/API/XRAnchor/delete)
    pub fn delete(&self) -> Undefined {
        self.inner.call("delete", &[]).as_::<Undefined>()
    }
}
