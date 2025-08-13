use super::*;




/// The XRDOMOverlayInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDOMOverlayInit {
    inner: Any,
}

impl FromVal for XRDOMOverlayInit {
    fn from_val(v: &Any) -> Self {
        XRDOMOverlayInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRDOMOverlayInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRDOMOverlayInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRDOMOverlayInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRDOMOverlayInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRDOMOverlayInit> for Any {
    fn from(s: XRDOMOverlayInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRDOMOverlayInit> for Any {
    fn from(s: &XRDOMOverlayInit) -> Any {
        s.inner.clone()
    }
}

impl XRDOMOverlayInit {
    /// Getter of the `root` attribute.
    pub fn root(&self) -> Element {
        self.inner.get("root").as_::<Element>()
    }

    /// Setter of the `root` attribute.
    pub fn set_root(&mut self, value: &Element) {
        self.inner.set("root", value);
    }
}
