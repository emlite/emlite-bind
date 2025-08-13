use super::*;




/// The XRPlane class.
/// [`XRPlane`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPlane {
    inner: Any,
}

impl FromVal for XRPlane {
    fn from_val(v: &Any) -> Self {
        XRPlane { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRPlane {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRPlane {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRPlane {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRPlane {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRPlane> for Any {
    fn from(s: XRPlane) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRPlane> for Any {
    fn from(s: &XRPlane) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRPlane);


impl XRPlane {
    /// Getter of the `planeSpace` attribute.
    /// [`XRPlane.planeSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane/planeSpace)
    pub fn plane_space(&self) -> XRSpace {
        self.inner.get("planeSpace").as_::<XRSpace>()
    }

}
impl XRPlane {
    /// Getter of the `polygon` attribute.
    /// [`XRPlane.polygon`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane/polygon)
    pub fn polygon(&self) -> TypedArray<DOMPointReadOnly> {
        self.inner.get("polygon").as_::<TypedArray<DOMPointReadOnly>>()
    }

}
impl XRPlane {
    /// Getter of the `orientation` attribute.
    /// [`XRPlane.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane/orientation)
    pub fn orientation(&self) -> XRPlaneOrientation {
        self.inner.get("orientation").as_::<XRPlaneOrientation>()
    }

}
impl XRPlane {
    /// Getter of the `lastChangedTime` attribute.
    /// [`XRPlane.lastChangedTime`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane/lastChangedTime)
    pub fn last_changed_time(&self) -> Any {
        self.inner.get("lastChangedTime").as_::<Any>()
    }

}
impl XRPlane {
    /// Getter of the `semanticLabel` attribute.
    /// [`XRPlane.semanticLabel`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlane/semanticLabel)
    pub fn semantic_label(&self) -> JsString {
        self.inner.get("semanticLabel").as_::<JsString>()
    }

}
