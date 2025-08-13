use super::*;




/// The DOMQuadInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMQuadInit {
    inner: Any,
}

impl FromVal for DOMQuadInit {
    fn from_val(v: &Any) -> Self {
        DOMQuadInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMQuadInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMQuadInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMQuadInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMQuadInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DOMQuadInit> for Any {
    fn from(s: DOMQuadInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMQuadInit> for Any {
    fn from(s: &DOMQuadInit) -> Any {
        s.inner.clone()
    }
}

impl DOMQuadInit {
    /// Getter of the `p1` attribute.
    pub fn p1(&self) -> DOMPointInit {
        self.inner.get("p1").as_::<DOMPointInit>()
    }

    /// Setter of the `p1` attribute.
    pub fn set_p1(&mut self, value: &DOMPointInit) {
        self.inner.set("p1", value);
    }
}
impl DOMQuadInit {
    /// Getter of the `p2` attribute.
    pub fn p2(&self) -> DOMPointInit {
        self.inner.get("p2").as_::<DOMPointInit>()
    }

    /// Setter of the `p2` attribute.
    pub fn set_p2(&mut self, value: &DOMPointInit) {
        self.inner.set("p2", value);
    }
}
impl DOMQuadInit {
    /// Getter of the `p3` attribute.
    pub fn p3(&self) -> DOMPointInit {
        self.inner.get("p3").as_::<DOMPointInit>()
    }

    /// Setter of the `p3` attribute.
    pub fn set_p3(&mut self, value: &DOMPointInit) {
        self.inner.set("p3", value);
    }
}
impl DOMQuadInit {
    /// Getter of the `p4` attribute.
    pub fn p4(&self) -> DOMPointInit {
        self.inner.get("p4").as_::<DOMPointInit>()
    }

    /// Setter of the `p4` attribute.
    pub fn set_p4(&mut self, value: &DOMPointInit) {
        self.inner.set("p4", value);
    }
}
